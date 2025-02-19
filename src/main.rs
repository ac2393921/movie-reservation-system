use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::task;

#[derive(Debug, Clone)]
struct TicketPurchaseEvent {
    user_id: String,
    seat_id: String,
}

#[tokio::main]
async fn main() {
    // バッファサイズ32の非同期チャンネルを作成
    let (tx, mut rx) = mpsc::channel::<TicketPurchaseEvent>(32);
    // イベントの送信側 (tx) を AppState に格納し、複数のタスクから参照できるようにする
    let state = AppState { tx: Arc::new(tx) };

    // ワーカーを起動
    tokio::spawn(event_worker(rx));
}

#[derive(Clone)]
struct AppState {
    tx: Arc<mpsc::Sender<TicketPurchaseEvent>>,
}

async fn event_worker(mut rx: mpsc::Receiver<TicketPurchaseEvent>) {
    while let Some(event) = rx.recv().await {
        println!("✅ Processing event: {:?}", event);
    }
}
