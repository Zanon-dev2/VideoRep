use gst::prelude::*;
use std::io;
use std::process::Command;
fn main() {
    gst::init().unwrap();
    loop{
        let mut VIDeo_nya = String::new();
        let mut enD_uwu = false;
        println!("escolha sua acao");
        if !In_nya(&mut VIDeo_nya, &mut enD_uwu){
            if enD_uwu {
                break;
            }
            continue;
        }
        let rep_nya = gst::ElementFactory::make("playbin")
            .name("pLayer_nya")
            .property("uri", VIDeo_nya)
            .build()
            .expect("Não foi possivel carregar o video");
        
        let VIDeosYnc_uwu = gst::ElementFactory::make("ximagesink")
            .name("VsYnc_nya")
            .build()
            .expect("Impossivel criar o sync");
        rep_nya.set_property("video-sink", &VIDeosYnc_uwu);
        rep_nya.set_state(gst::State::Playing).unwrap();
        let _ = Command::new("sh")
            .arg("-c")
            .arg("sleep 0.6 && \
                  SCREEN_W=$(xdotool getdisplaygeometry | awk '{print $1}') && \
                  SCREEN_H=$(xdotool getdisplaygeometry | awk '{print $2}') && \
                  WINDOW_ID=$(xdotool search --onlyvisible --name 'losloslos' 2>/dev/null || xdotool search --onlyvisible --classname 'losloslos' 2>/dev/null || xdotool getactivewindow) && \
                  if [ ! -z \"$WINDOW_ID\" ]; then \
                      xdotool windowsize $WINDOW_ID 854 480 && \
                      eval $(xdotool getwindowgeometry --shell $WINDOW_ID) && \
                      TARGET_X=$(( (SCREEN_W - WIDTH) / 2 )) && \
                      TARGET_Y=$(( (SCREEN_H - HEIGHT) / 2 )) && \
                      xdotool windowmove $WINDOW_ID $TARGET_X $TARGET_Y; \
                  fi")
            .spawn();
        let bUs_uwu = rep_nya.bus().unwrap();
        for msg_uwu in bUs_uwu.iter_timed(gst::ClockTime::NONE){
            use gst::MessageView;
            match msg_uwu.view(){
                MessageView::Eos(..) => {
                    println!("acabou");
                    break;
                },
                MessageView::Error(err) => {
                    println!("Deu não chefia {}", err.error());
                    break;
                },
                _ => {},
            }
        }
        rep_nya.set_state(gst::State::Null).unwrap();
    }
}
fn In_nya(WaY_nya: &mut String, end: &mut bool)->bool{
    let mut nomeesP_nya = String::new();
    io::stdin()
        .read_line(&mut nomeesP_nya)
        .expect("desconhecido");
    let nome_nya = nomeesP_nya.trim();
    let mut escoLHa_uwu = String::new();
    match nome_nya {
        "losloslos" => {
            *WaY_nya = "file:///mnt/c/Users/User/Videos/Saga%20of%20Tanya%20the%20Evil%20-%20Ending%20%20%20Los!%20Los!%20Los!.mp4".to_string();
            true
        },
        "Ugoku" => {
            *WaY_nya = "file:///mnt/c/Users/User/Videos/4K%20Video%20Downloader+/ugoku.mp4".to_string();
            true
        },
        "OneMoreNight" => {
            *WaY_nya = "file:///mnt/c/Users/User/Videos/4K%20Video%20Downloader+/OneMoreNight.mp4".to_string();
            true
        },
        "Departure" => {
            *WaY_nya = "file:///mnt/c/Users/User/Videos/4K%20Video%20Downloader+/Hunter%20X%20Hunter%20Opening%201%20%20%20Departure!.mp4".to_string();
            true
        }
        "list" => {
            println!("losloslos\nUgoku\nOneMoreNight\nDeparture");
            false
        },
        "end" => {
            *end = true;
            false
        },
        _ => {
            println!("desconhecido, digite list"); 
            false
        },
    }
}