use app_core::StringListApp;
use futures_util::StreamExt;
use std::{
    collections::BTreeSet,
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};
use tokio::time::timeout;

use template_rust_web_app::memory_list::InMemoryList;

#[tokio::test]
async fn readers_and_subscribers_see_the_same_append() {
    let app = StringListApp::new(InMemoryList::new(vec!["seed".into()]), || {
        "new-string".into()
    });
    let mut first = app.subscribe();
    let mut second = app.subscribe();

    assert_eq!(app.read().as_slice(), ["seed"]);
    for subscriber in [&mut first, &mut second] {
        let initial = timeout(Duration::from_secs(2), subscriber.next())
            .await
            .expect("initial snapshot")
            .unwrap();
        assert_eq!(initial.as_slice(), ["seed"]);
    }

    assert_eq!(app.add_random_string(), "new-string");
    assert_eq!(app.read().as_slice(), ["seed", "new-string"]);
    for subscriber in [&mut first, &mut second] {
        let updated = timeout(Duration::from_secs(2), subscriber.next())
            .await
            .expect("updated snapshot")
            .unwrap();
        assert_eq!(updated, app.read());
    }
}

#[tokio::test]
async fn concurrent_appends_all_appear_in_the_latest_snapshot() {
    let next = AtomicUsize::new(0);
    let app = StringListApp::new(InMemoryList::new(Vec::new()), move || {
        format!("item-{}", next.fetch_add(1, Ordering::Relaxed))
    });
    let mut updates = app.subscribe();

    assert!(updates.next().await.unwrap().is_empty());
    std::thread::scope(|scope| {
        for _ in 0..2 {
            scope.spawn(|| {
                for _ in 0..20 {
                    app.add_random_string();
                }
            });
        }
    });

    let latest = timeout(Duration::from_secs(2), updates.next())
        .await
        .expect("snapshot after appends")
        .unwrap();
    assert_eq!(latest, app.read());
    assert_eq!(latest.len(), 40);
    let expected: BTreeSet<_> = (0..40).map(|index| format!("item-{index}")).collect();
    assert_eq!(latest.iter().cloned().collect::<BTreeSet<_>>(), expected);
}
