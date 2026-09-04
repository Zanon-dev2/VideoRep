use gst::prelude::*;
use std::io;
use std::process::Command;
//the strange way that I give the var names is because I created my own case just for Rust -> FemboyCas
//it has two ways to write, the first one is to write the letters that are tall n long like L, I, V, U, in upper case while letters like m, n, b, c that doesn't have tall long things are lowercase
//this thing is because femboys come with a "surprise" together so the long n tall letters represent this and must have a _nya or _uwu ate the end of the name

//the second way is the same thing but we write _ to represent the long striped socks EG: J_U_s_T_nya
//EG: game_uwu
//I prefer the first way because is boring to have to long names just because of _
fn main() {
    gst::init().unwrap();
    loop{
        let mut VIDeo_nya = String::new();
        let mut enD_uwu = false;
        println!("chose");
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
            .expect("Coundn't load the video");
        
        let VIDeosYnc_uwu = gst::ElementFactory::make("ximagesink")
            .name("VsYnc_nya")
            .build()
            .expect("error while making the sync");
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
                    println!("end");
                    break;
                },
                MessageView::Error(err) => {
                    println!("Erro: {}", err.error());
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
        .expect("Unknown");
    let nome_nya = nomeesP_nya.trim();
    let mut escoLHa_uwu = String::new();
    match nome_nya {
        "losloslos" => {
            *WaY_nya = "file:///ur/directory/Saga%20of%20Tanya%20the%20Evil%20-%20Ending%20%20%20Los!%20Los!%20Los!.mp4".to_string();
            true
        },
        "Ugoku" => {
            *WaY_nya = "file:///ur/directory/ugoku.mp4".to_string();
            true
        },
        "OneMoreNight" => {
            *WaY_nya = "file:///ur/directory/OneMoreNight.mp4".to_string();
            true
        },
        "Departure" => {
            *WaY_nya = "file:///ur/directory/Hunter%20X%20Hunter%20Opening%201%20%20%20Departure!.mp4".to_string();
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
