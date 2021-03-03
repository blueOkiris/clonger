/*
 * Global space for application constants
 */

namespace Clonger.Data {
    static class AppSettings {
        public static readonly (int, int) DefaultWindowSize = (1280, 720);
        public static readonly (int, int) MinWindowSize = (600, 400);
        public static readonly string WindowTitle = "Clonger";
        public static readonly uint Margin = 5;
        public static readonly uint BorderWidth = 5;
        public static readonly int MaxHotKeysPerRow = 8;
        public static readonly int DefaultDocFontSize = 14;
        public static readonly int SubtitleDocFontSize = 28;
        public static readonly int TitleDocFontSize = 40;
    }
}
