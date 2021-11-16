//! FFmpeg 模組的格式 enum 部分。
use std::fmt::Formatter;

/// FFmpeg 的目標格式
pub enum Format {
    /// MP3 格式。
    ///
    /// 最通用的格式，但音質欠佳。
    /// 第一項是位元速率。
    ///
    /// # 範例
    ///
    /// ```
    /// let mp3_format = Format::Mp3(320);
    /// ```
    Mp3(usize),
    /// AAC 格式。
    ///
    /// 與 Apple Music 同格式，是有損壓縮，但音質較 MP3 佳。
    /// 第一項是位元速率。
    ///
    /// # 範例
    ///
    /// ```
    /// let aac_format = Format::Aac(128);
    /// ```
    Aac(usize),
    /// FLAC 格式。
    ///
    /// 無損壓縮音質。能夠保存完整音訊細節，
    /// 但可能比較耗費 CPU 資源及流量。
    ///
    /// # 範例
    ///
    /// ```
    /// let flac_format = Format::Flac;
    /// ```
    Flac,
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let fmt = match *self {
            Format::Mp3(_) => "mp3",
            Format::Aac(_) => "aac",
            Format::Flac => "flac",
        };

        write!(f, "{}", fmt)
    }
}

impl Format {
    /// 取得格式指定的位元速率（若有指定且支援）。
    pub fn get_bitrate(&self) -> Option<usize> {
        match *self {
            Format::Mp3(bitrate) | Format::Aac(bitrate) => Some(bitrate),
            _ => None,
        }
    }

    /// 取得格式指定的 codec。
    pub fn get_codec(&self) -> &'static str {
        match *self {
            Format::Mp3(_) => "libmp3lame",
            Format::Aac(_) => "aac",
            Format::Flac => "flac",
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::services::ffmpeg::format::Format;

    #[test]
    fn format_fmt_test() {
        let fmt = |text| format!("{}", text);

        assert_eq!(fmt(Format::Mp3(320)), "mp3");
        assert_eq!(fmt(Format::Aac(160)), "aac");
        assert_eq!(fmt(Format::Flac), "flac");
    }

    #[test]
    fn get_bitrate_test() {
        assert_eq!(Format::Mp3(320).get_bitrate(), Some(320));
        assert_eq!(Format::Aac(160).get_bitrate(), Some(160));
        assert_eq!(Format::Flac.get_bitrate(), None);
    }

    #[test]
    fn get_codec_test() {
        assert_eq!(Format::Mp3(320).get_codec(), "libmp3lame");
        assert_eq!(Format::Aac(160).get_codec(), "aac");
        assert_eq!(Format::Flac.get_codec(), "flac");
    }
}
