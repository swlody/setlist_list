use loco_rs::testing;
use serial_test::serial;
use setlist_list::app::App;

#[tokio::test]
#[serial]
async fn can_request_root() {
    testing::request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/api/hello").await;
        assert_eq!(res.status_code(), 200);
        assert_eq!(res.text(), "<h1>hello</h1>");
    })
    .await;
}
