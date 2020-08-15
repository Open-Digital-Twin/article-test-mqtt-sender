use rumqttc::{EventLoop, MqttOptions, Publish, QoS, Request};
use std::time::Duration;
use tokio::{task, time};
use std::error::Error;
use std::env;
use rand::{distributions::Alphanumeric, Rng, thread_rng};

#[tokio::main(core_threads = 1)]
async fn main() -> Result<(), Box<dyn Error>> {

let args : Vec<String> = env::args().collect();

    let msg_size = &args[1];
    let delay = &args[2];
    let adress = &args[3];


    let size = msg_size.parse::<usize>().unwrap();
    let delay = delay.parse::<u64>().unwrap();

    
    let mut mqttoptions = MqttOptions::new("Sender", adress, 1883);
    mqttoptions.set_keep_alive(50);
    mqttoptions.set_max_packet_size(2);

    let mut eventloop = EventLoop::new(mqttoptions, 10).await;
    let requests_tx = eventloop.handle();

    task::spawn(async move {
        let mut index : i32 = 0;
        loop{
            
            let index_str = index.to_string();
            let mut payload: String = std::iter::repeat(())
            .map(|()| thread_rng().sample(Alphanumeric))
            .take(size - index_str.len() - 1).collect();
  
          payload.insert_str(0, &" ");
          payload.insert_str(0, &index_str);

        requests_tx.send(publish_request(&(payload.as_str()) ,"hello")).await.unwrap();
        index += 1;
        time::delay_for(Duration::from_millis(delay)).await;
    
        }
    
    });

    loop {
        let _communication = eventloop.poll().await?;
        }  

}


fn publish_request(payload: &str, topic: &str) -> Request {
    let topic = topic.to_owned();
    let message = String::from(payload);
  
    let qos = QoS::AtLeastOnce;
  
    let publish = Publish::new(&topic, qos, message);
    Request::Publish(publish)
  }