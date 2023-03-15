use std::{
    vec::Vec,
    sync::{
        Arc,
        Mutex
    }, io::stdin,
};
use tokio::net::{TcpListener,TcpStream};
use tokio::io::{self,AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main()
{
    let listener = TcpListener::bind("192.168.1.11:7878").await.unwrap();
    let finish = Arc::new(Mutex::new(false));
    let finish_thread = finish.clone();
    tokio::spawn(async move
    {
        let finish = finish_thread;
        let stdin = stdin();
        let mut buf = String::new();
        loop
        {
            stdin.read_line(&mut buf).unwrap();
            if buf.trim() == "close"
            {
                *finish.lock().unwrap() = true;
                return
            }
            else
            {
                buf.clear();
            }
        }
    });
    tokio::spawn(async move
    {
        loop
        {
            let mut players =Vec::<TcpStream>::new();
            loop
            {
                let (socket, _) = listener.accept().await.unwrap();
                let acepted = handle_connection(socket).await;
                match acepted
                {
                    Some(con) => players.push(con),
                    None => (),
                }
                if players.len() == 2
                {
                    break;
                }
            }

            let mut player2 = players.pop().unwrap();
            let mut player1 = players.pop().unwrap();
            drop(players);
            {//envia que turno corresponde a cada jugador
                let turno_player1 = &[0];
                let turno_player2 = &[1];

                player1.write(turno_player1).await.unwrap();
                player2.write(turno_player2).await.unwrap();
            }
            //canales para recibir y reenviar los tableros
            let (mut rd1,mut wr1) = io::split(player1);
            let (mut rd2,mut wr2) = io::split(player2);

            tokio::spawn(async move
            {
                let mut buf = [0u8;10];
                loop
                {
                    let n = rd1.read(&mut buf).await.unwrap_or(1);
                    if n == 1
                    {
                        
                        break;
                    }
                    else
                    {
                        wr2.write(&buf).await.unwrap();
                        println!("write to player 2: {:?}",buf);
                    }
                }
                println!("player1: Disconnected");
            });

            tokio::spawn(async move
            {
                let mut buf =  [0u8;10];
                loop
                {
                    let n = rd2.read(&mut buf).await.unwrap_or(1);
                    if n == 1 {
                        
                        break;
                    }
                    else
                    {
                        wr1.write(&buf).await.unwrap();
                        println!("write to player 1: {:?}",buf);
                    }
                }
                println!("player 2: Disconnected");
            });

        }
    });
    loop
    {
        let f = finish.lock().unwrap();
        if *f == true
        {
            return
        }
    }
}

async fn handle_connection(mut stream: TcpStream) -> Option<TcpStream>{
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).await.unwrap();
    if String::from_utf8_lossy(&buffer[..]).eq(&String::from("conection request")) {
        return None;
    }
    println!("Accepting {}", stream.peer_addr().unwrap());
    Some(stream)
}