use crate::viz::plot::Plot;
use std::process::Command;

pub struct Video {
    frame_count: usize,
    scale: usize,
}

impl Video {
    pub fn add_frame(&mut self, plot: &Plot) {
        let file_name = format!("frame_{:0>4}.png", self.frame_count);

        plot.save(&file_name, self.scale);

        self.frame_count += 1;
    }

    pub fn render(&self) {
        let v = Command::new("ffmpeg")
            .arg("-i")
            .arg("frame_%04d.png")
            .arg("-c:v")
            .arg("libx264")
            .arg("-r")
            .arg("60")
            .arg("out.mp4")
            .output()
            .expect("ls command failed to start");
        println!("{}", String::from_utf8_lossy(&v.stderr));
    }

    pub fn new(scale: usize) -> Video {
        Video {
            frame_count: 0,
            scale,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::viz::plot::Plot;
    use crate::viz::video::Video;

    #[test]
    fn test() {
        let mut v = Video::new(4);

        for c in 0..8 {
            let mut p = Plot::new(16, 16);

            for x in 0..c {
                p.draw(x, 5, (255, 0, 0));
            }

            v.add_frame(&p);
        }

        v.render();
    }
}
