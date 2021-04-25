use atomic_batcher::Batcher;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

#[test]
#[should_panic(expected = "Task called")]
fn test_run_single_batch() {
  let task = |_| async { panic!("Task called") };
  let batch = Batcher::new(task);
  batch.append(());
}

#[test]
fn test_batches_are_generic() {
  let say_hello = |vs| async move {
    for s in vs {
      println!("Hello {}", s)
    }
  };
  let batch = Batcher::new(say_hello);
  batch.append("world");

  let count_up_to = |vs| async move {
    for n in vs {
      println!("{:?}", (1..=n).collect::<Vec<u8>>());
    }
  };
  let batch = Batcher::new(count_up_to);
  batch.append(10);
}

#[test]
#[should_panic(expected = "Callback called")]
fn test_callback_is_called() {
  let do_nothing = |_| async {};
  let batch = Batcher::new(do_nothing);
  let callback = |_| async { panic!("Callback called") };

  batch.appendcb((), callback);
}

#[test]
fn test_result_is_propagated() {
  let echo_input = |s| async move { return s };
  let batch = Batcher::new(echo_input);
  let callback = |s| async move { assert_eq!(s, vec!["Some MSG"]) };

  batch.appendcb("Some MSG", callback);
}

async fn toggle_bool(vb: Vec<&mut bool>) -> String {
  for b in vb {
    *b = !(*b);
  }
  return "".to_string();
}

#[test]
fn test_allow_some_mutability() {
  let mut toggle = false;
  let batch = Batcher::new(toggle_bool);
  batch.append(&mut toggle);
  assert_eq!(toggle, true);
}

#[test]
fn test_batches_block_thread() {
  let test_start = Instant::now();
  let sleep = |vmillis| async move {
    for millis in vmillis {
      sleep(Duration::from_millis(millis));
    }
  };

  let batch = Batcher::new(sleep);
  batch.append(100);
  batch.append(100);
  assert!(test_start.elapsed() >= Duration::from_millis(200));
}
