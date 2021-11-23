use std::thread;
use std::sync::mpsc;
fn writer(chan: mpsc::SyncSender<u32>) -> () {
    let mut cur: u32 = 0;
    while let Ok(()) = chan.send(cur) {
        cur = cur.wrapping_add(1);
        if cur % 1_000_000 == 0 { println!("Writer > cur = {}",cur);}
    }
}
fn reader(read_limit: usize, chan: mpsc::Receiver<u32>) -> () {
    let mut cur: u32 = 0;
    while (cur as usize) < read_limit {
        let num = chan.recv().unwrap();
        assert_eq!(num, cur);
        if num % 1_000_000 == 0 { println!("Reader > num = {}",num);}
        cur = cur.wrapping_add(1);
    }
}
fn main() {
    let capacity = 10;
    let read_limit = 10_000_000;
    let (snd, rcv) = mpsc::sync_channel(capacity);
    let reader_jh = thread::spawn(move || {
        reader(read_limit, rcv);
    });
    let _writer_jh = thread::spawn(move || {
        writer(snd);
    });
    reader_jh.join().unwrap();
}