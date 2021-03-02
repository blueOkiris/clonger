/*
 * This is a data type that stores
 *  1) A hotkey used in the IPA view with alt
 *  2) A list of all the characters the hotkey provides
 */

using System;
using Gdk;

namespace Clonger.Data {
    struct HotKey {
        public Key key;
        public string[] symbols;
        
        public static HotKey[] HotKeys = {
            new HotKey {
                key = Key.A,
                symbols = new string[] { "ɑ", "æ", "ɐ", "ɑ̃" }
            }, new HotKey {
                key = Key.B,
                symbols = new string[] { "β", "ɓ", "ʙ" }
            }, new HotKey {
                key = Key.C,
                symbols = new string[] { "ç", "ɕ" }
            }, new HotKey {
                key = Key.D,
                symbols = new string[] { "ð", "d͡ʒ", "ɖ", "ɗ" }
            }, new HotKey {
                key = Key.E,
                symbols = new string[] { "ə", "ɚ", "ɵ", "ɘ" }
            }, new HotKey {
                key = Key.Key_3,
                symbols = new string[] { "ɛ", "ɜ", "ɝ", "ɛ̃", "ɞ" }
            }, new HotKey {
                key = Key.G,
                symbols = new string[] { "ɠ", "ɢ", "ʛ" }
            }, new HotKey {
                key = Key.H,
                symbols = new string[] { "ħ", "ɦ","ɥ", "ɧ", "ʜ" }
            }, new HotKey {
                key = Key.I,
                symbols = new string[] { "ɪ", "ɨ", "ɪ̈" }
            }, new HotKey {
                key = Key.J,
                symbols = new string[] { "ʝ", "ɟ", "ʄ" }
            }, new HotKey {
                key = Key.L,
                symbols = new string[] { "ɫ", "ɭ", "ɬ", "ʟ", "ɮ" }
            }, new HotKey {
                key = Key.M,
                symbols = new string[] { "ɱ" }
            }, new HotKey {
                key = Key.N,
                symbols = new string[] { "ŋ", "ɲ", "ɳ", "ɴ" }
            }, new HotKey {
                key = Key.O,
                symbols = new string[] { "ɔ","œ", "ɒ", "ɔ̃", "ɶ" }
            }, new HotKey {
                key = Key.Key_0,
                symbols = new string[] { "ø" }
            }, new HotKey {
                key = Key.P,
                symbols = new string[] { "ɸ" }
            }, new HotKey {
                key = Key.R,
                symbols = new string[] { "ɾ", "ɹ", "ʁ", "ʀ", "ɻ", "ɽ", "ɺ" }
            }, new HotKey {
                key = Key.S,
                symbols = new string[] { "ʃ", "ʂ" }
            }, new HotKey {
                key = Key.T,
                symbols = new string[] { "θ", "t͡ʃ", "t͡s", "ʈ" }
            }, new HotKey {
                key = Key.U,
                symbols = new string[] { "ʊ", "ʉ" }
            }, new HotKey {
                key = Key.V,
                symbols = new string[] { "ʌ", "ʋ", "ⱱ" }
            }, new HotKey {
                key = Key.W,
                symbols = new string[] { "ɯ", "ʍ", "ɰ" }
            }, new HotKey {
                key = Key.X,
                symbols = new string[] { "χ" }
            }, new HotKey {
                key = Key.Y,
                symbols = new string[] { "ɣ", "ʎ", "ʏ", "ɤ" }
            }, new HotKey {
                key = Key.Z,
                symbols = new string[] { "ʒ", "ʐ", "ʑ" }
            }, new HotKey {
                key = Key.Key_2,
                symbols = new string[] { "ʔ", "ʕ", "ʡ", "ʢ" }
            }, new HotKey {
                key = Key.Q,
                symbols = new string[] { "ˈ", "ˌ" }
            }, new HotKey {
                key = Key.backslash,
                symbols = new string[] { "ǀ", "ǁ", "ǂ", "ǃ", "ʘ" }
            }, new HotKey {
                key = Key.colon,
                symbols = new string[] { "ː" }
            }, new HotKey {
                key = Key.period,
                symbols = new string[] {
                    "˩", "˨", "˧", "˦", "˥", "˥˩", "˩˥", "˩˨", "˦˥", "˧˦˧"
                }
            }, new HotKey {
                key = Key.grave,
                symbols = new string[] {
                    "ˤ", "ˀ", "ᵝ", "ᵊ", "ʱ", "ˡ", "ⁿ", "ʳ", "ᵗ", "ˠ", "ʼ"
                }
            },
        };
    }
}