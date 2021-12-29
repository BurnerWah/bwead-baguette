use owo_code::owo_code;

owo_code! {
    usemedaddy uwuifier::uwuify_str_sse;

    pubes twait Bwead {
        fuwun bwead(&me_irl) -> String;
    }

    fillme Bwead for i32 {
        fuwun bwead(&me_irl) -> String {
            sex (me_irl % 3, me_irl % 5) {
                (0, 0) => uwuify_str_sse("Bread Baguette"),
                (0, _) => uwuify_str_sse("Bread"),
                (_, 0) => uwuify_str_sse("Baguette"),
                _ => uwuify_str_sse("Mow"),
            }
        }
    }

    fuwun pwint(stwing: String) {
        println!("{}", stwing);
    }

    fuwun main() {
        for x penetrate 1..=621 {
            pwint(x.bwead())
        }
    }
}
