// This is a part of rust-encoding.
// Copyright (c) 2013-2014, Kang Seonghoon.
// See README.md and LICENSE.txt for details.

//! Macros and utilities for testing.

#![macro_escape]

macro_rules! assert_feed_ok(
    ($this:expr, $processed:expr, $unprocessed:expr, $output:expr) => ({
        let processed = $processed;
        let processed = $this.test_norm_input(processed);
        let unprocessed = $unprocessed;
        let unprocessed = $this.test_norm_input(unprocessed);
        let output = $output;
        let output = $this.test_norm_output(output);
        let input = $this.test_concat(processed, unprocessed);
        let (nprocessed, err, buf) = $this.test_feed(input.as_slice());
        let upto = err.map(|e| e.upto);
        assert!(processed.len() == nprocessed && None == upto,
                "raw_feed should return {:?}, but instead returned {:?}",
                (processed.len(), None::<uint>), (nprocessed, upto));
        assert!(output == buf.as_slice(),
                "raw_feed should push {:?}, but instead pushed {:?}", output, buf.as_slice());
    })
)

macro_rules! assert_feed_err(
    ($this:expr, $processed:expr, $problem:expr, $remaining:expr, $output:expr) => ({
        let processed = $processed;
        let processed = $this.test_norm_input(processed);
        let problem = $problem;
        let problem = $this.test_norm_input(problem);
        let remaining = $remaining;
        let remaining = $this.test_norm_input(remaining);
        let output = $output;
        let output = $this.test_norm_output(output);
        let input = $this.test_concat($this.test_concat(processed, problem).as_slice(), remaining);
        let (nprocessed, err, buf) = $this.test_feed(input.as_slice());
        let upto = err.map(|e| e.upto);
        assert!(processed.len() == nprocessed && Some(processed.len() + problem.len()) == upto,
                "raw_feed should return {:?}, but instead returned {:?}",
                (processed.len(), Some(processed.len() + problem.len())), (nprocessed, upto));
        assert!(output == buf.as_slice(),
                "raw_feed should push {:?}, but instead pushed {:?}", output, buf.as_slice());
    })
)

macro_rules! assert_finish_ok(
    ($this:expr, $output:expr) => ({
        let output = $output;
        let output = $this.test_norm_output(output);
        let (err, buf) = $this.test_finish();
        let upto = err.map(|e| e.upto);
        assert!(None == upto,
                "raw_finish should return {:?}, but instead returned {:?}", None::<uint>, upto);
        assert!(output == buf.as_slice(),
                "raw_finish should push {:?}, but instead pushed {:?}", output, buf.as_slice());
    })
)

macro_rules! assert_finish_err(
    ($this:expr, $output:expr) => ({
        let output = $output;
        let output = $this.test_norm_output(output);
        let (err, buf) = $this.test_finish();
        let upto = err.map(|e| e.upto);
        assert!(Some(0) == upto,
                "raw_finish should return {:?}, but instead returned {:?}", Some(0), upto);
        assert!(output == buf.as_slice(),
                "raw_finish should push {:?}, but instead pushed {:?}", output, buf.as_slice());
    })
)

/// Some Korean text to test.
//
// the first paragraphs of the article "Korean Language" from Korean Wikipedia.
// https://ko.wikipedia.org/w/index.php?title=%ED%95%9C%EA%B5%AD%EC%96%B4&oldid=12331875
pub static KOREAN_TEXT: &'static str =
    "한국어(韓國語)는 주로 한반도(韓半島)와 한민족(韓民族) 거주 지역에서 쓰이는 언어로, \
     대한민국에서는 한국어, 한국말이라고 부르고, 조선민주주의인민공화국과 중국, 일본에서는 \
     조선어(朝鮮語), 조선말이라고 불린다. 우즈베키스탄, 러시아 등 구 소련의 고려인들 사이에서는 \
     고려말(高麗語)로 불린다. 19세기 중반 이후 한반도와 주변 정세의 혼란, 20세기 전반 \
     일본 제국주의의 침략, 20세기 후반 대한민국의 해외 이민에 의해 중국 동북 지방, 일본, \
     러시아 연해주와 사할린, 우즈베키스탄, 미국, 캐나다, 오스트레일리아, 필리핀, 베트남, 브라질 등 \
     세계 곳곳에 한민족이 이주하면서 한국어가 쓰이고 있다. 한국어 쓰는 인구는 전 세계를 통틀어 \
     약 8천250만 명으로 추산된다.";

/// Some Japanese text to test.
//
// the first paragraphs of the article "Japanese Language" from Japanese Wikipedia.
// https://ja.wikipedia.org/w/index.php?title=%E6%97%A5%E6%9C%AC%E8%AA%9E&oldid=51443986
pub static JAPANESE_TEXT: &'static str =
    "日本語（にほんご、にっぽんご）とは、主に日本国内や日本人同士の間で使われている言語である。\
     日本は法令によって「公用語」を規定していないが、法令その他の公用文は日本語で記述され、\
     各種法令（裁判所法第74条、会社計算規則第57条、特許法施行規則第2条など）において\
     日本語を用いることが定められるなど事実上の公用語となっており、学校教育の「国語」でも\
     教えられる。使用人口について正確な統計はないが、日本国内の人口、および日本国外に住む\
     日本人や日系人、日本がかつて統治した地域の一部の住民など、約1億3千万人以上と考えられる。\
     統計によって前後する可能性はあるが、この数は世界の母語話者数で上位10位以内に入る人数である。";

/// Some simplified Chinese text to test.
//
// the first paragraphs of the article "Chinese Language" from Chinese Wikipedia.
// https://zh.wikipedia.org/w/index.php?title=%E6%B1%89%E8%AF%AD&variant=zh-cn&oldid=31224104
pub static SIMPLIFIED_CHINESE_TEXT: &'static str =
    "汉语，又称中文、华语（东南亚）、国语（中华民国国语）、中国语（日本、韩国等），\
     其他名称有汉文（通常指文言文）、华文、唐文、唐话、中国话等，是属汉藏语系的分析语，具有声调。\
     汉语的文字系统——汉字是一种意音文字，表意的同时也具一定的表音功能。\
     汉语包含书面语以及口语两部分，古代书面汉语称为文言文，现代书面汉语一般指使用现代标准汉语语法，\
     词汇的中文通行文体。目前全球有六分之一人口使用汉语作为母语。现代汉语书面语高度统一，\
     口语则有官话、粤语、吴语、湘语、赣语、客家语、闽语等七种主要汉语言\
     （也有人认为晋语和（或）徽语和（或）平话（广西平话）也应为独立汉语言，\
     也有其他人认为闽语其实是一个语族，下辖闽南语、闽东语、闽中语以及莆仙语，\
     国际标准化组织即持此观点，部分资料将其中的一至六种也算成单独的汉语言，\
     这就是八至十三种汉语言的由来）。";

/// Some traditional Chinese text to test.
//
// the first paragraphs of the article "Chinese Language" from Chinese Wikipedia.
// https://zh.wikipedia.org/w/index.php?title=%E6%B1%89%E8%AF%AD&variant=zh-tw&oldid=31224104
pub static TRADITIONAL_CHINESE_TEXT: &'static str =
    "漢語，又稱中文、華語（東南亞）、國語（中華民國國語）、中國語（日本、韓國等），\
     其他名稱有漢文（通常指文言文）、華文、唐文、唐話、中國話等，是屬漢藏語系的分析語，具有聲調。\
     漢語的文字系統——漢字是一種意音文字，表意的同時也具一定的表音功能。\
     漢語包含書面語以及口語兩部分，古代書面漢語稱為文言文，現代書面漢語一般指使用現代標準漢語語法，\
     詞彙的中文通行文體。目前全球有六分之一人口使用漢語作為母語。現代漢語書面語高度統一，\
     口語則有官話、粵語、吳語、湘語、贛語、客家語、閩語等七種主要漢語言\
     （也有人認為晉語和（或）徽語和（或）平話（廣西平話）也應為獨立漢語言，\
     也有其他人認為閩語其實是一個語族，下轄閩南語、閩東語、閩中語以及莆仙語，\
     國際標準化組織即持此觀點，部分資料將其中的一至六種也算成單獨的漢語言，\
     這就是八至十三種漢語言的由來）。";
