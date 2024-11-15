fn main() {
    let search_term = "determination";
    let quote = "\
If you set yourself to your present task along the path of true reason, with all determination, 
vigour, and good will: if you admit no distraction, but keep your own divinity pure and 
standing strong, as if you had to surrender it right now; if you grapple this to you, 
expecting nothing, shirking nothing, but self-content with each present action taken in 
accordance with nature and a heroic truthfulness in all that you say and mean 
then you will lead a good life. And nobody is able to stop you";

for (i ,line) in quote.lines().enumerate(){
        if line.contains(search_term){
            let line_num = i+1;
            println!("{}: {}", line_num, line);
        }
    }
}
