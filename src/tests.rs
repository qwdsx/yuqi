
#[cfg(test)]
mod tests {
    use crate::parser::*;

    #[test]
    fn test_replace_v_with_umlaut() {
        let word1 = "nv";
        let word2 = "nve";

        dbg!(replace_v_with_umlaut(word1));
        dbg!(replace_v_with_umlaut(word2));

        assert_eq!(replace_v_with_umlaut(word1), "nü");
        assert_eq!(replace_v_with_umlaut(word2), "nüe");
    }

    #[test]
    fn test_remove_numbers() {
        let words = ["yue4gong1zhu3", "ya2", "xue3", "zai4jian4"];
        let numbers_removed = ["yuegongzhu", "ya", "xue", "zaijian"];

        let removed1 = remove_numbers(words[0]);
        let removed2 = remove_numbers(words[1]);
        let removed3 = remove_numbers(words[2]);
        let removed4 = remove_numbers(words[3]);
        
        assert_eq!(removed1, numbers_removed[0]);
        assert_eq!(removed2, numbers_removed[1]);
        assert_eq!(removed3, numbers_removed[2]);
        assert_eq!(removed4, numbers_removed[3]);
    }

    #[test]
    fn test_get_syllables() {
        let words = ["yue4gong1zhu3", "ya2", "xue3", "zai4jian4", "xi3huan"];
        let syllable_count = [3, 1, 1, 2, 2];

        let syllable1 = get_syllables(words[0]);
        let syllable2 = get_syllables(words[1]);
        let syllable3 = get_syllables(words[2]);
        let syllable4 = get_syllables(words[3]);
        let syllable5 = get_syllables(words[4]);

        dbg!(&syllable1);
        dbg!(&syllable2);
        dbg!(&syllable3);
        dbg!(&syllable4);
        dbg!(&syllable5);

        assert_eq!(syllable1.len(), syllable_count[0]);
        assert_eq!(syllable2.len(), syllable_count[1]);
        assert_eq!(syllable3.len(), syllable_count[2]);
        assert_eq!(syllable4.len(), syllable_count[3]);
        assert_eq!(syllable5.len(), syllable_count[4]);
    }

    #[test]
    fn test_get_tone() {
        let words = ["gong1", "ya2", "xue3", "jiao4"];
        for (i, w) in words.iter().enumerate() {
            let tone = get_tone(w);
            assert_eq!(tone, (i + 1) as u8);
        }
    }

    #[test]
    fn test_check_vowel_priority() {
        let word1 = "yue";
        let word2 = "yao";
        let word3 = "zhuan";
        let word4 = "liao";

        assert_eq!(check_vowel_priority(word1), 'e');
        assert_eq!(check_vowel_priority(word2), 'a');
        assert_eq!(check_vowel_priority(word3), 'a');
        assert_eq!(check_vowel_priority(word4), 'a');
    }

    #[test]
    fn test_add_tones() {
        let test1 = "qi1";
        let test2 = "cheng2";
        let test3 = "mu3";
        let test4 = "yao4";
        let test5 = "yue4";
        let test6 = "wo3 xi3huan ni3";

        assert_eq!(add_tones(test1).as_str(), "qī");
        assert_eq!(add_tones(test2).as_str(), "chéng");
        assert_eq!(add_tones(test3).as_str(), "mǔ");
        assert_eq!(add_tones(test4).as_str(), "yào");
        assert_eq!(add_tones(test5).as_str(), "yuè");
        assert_eq!(add_tones(test6).as_str(), "wǒ xǐhuan nǐ");
    }


}