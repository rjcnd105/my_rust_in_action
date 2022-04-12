fn main() {
    let ctx_lines = 2;
    let needle = "is";
    let haystack = "\
 Every face, every shop,
 bedroom window, public-house, and
 dark square is a picture
 feverishly turned--in search of what?
 It is the same with books.
 What do we seek
 through millions of pages?";

    // tags는  needle문자가 포함된 열의 인덱싱을 기억한다.
    let mut tags: Vec<usize> = vec![];

    // (usize, String) 튜플을 저장하는 벡터를 저장하는 벡터
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];

    for (i, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            tags.push(i);

            // 크기을 지정해서 벡터를 생성
            // 이렇게 하면 메모리 할당 횟수가 줄어들어 메모리 절약 가능
            let v = Vec::with_capacity(2 * ctx_lines + 1);
            ctx.push(v);
        }
    }
    println!("tags: {:?}", tags);

    if tags.is_empty() {
        return;
    }

    for (i, line) in haystack.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag + ctx_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i;
            println!("{}: {}", line_num, line);
        }
    }
}
