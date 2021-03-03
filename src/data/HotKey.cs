/*
 * This is a data type that stores
 *  1) A hotkey used in the IPA view with alt
 *  2) A list of all the characters the hotkey provides
 *
 * It contains a static member that contains all the hotkeys laid out
 * I could put those in settings, but I thought they fit better here
 */

using System;
using Gdk;

namespace Clonger.Data {
    struct HotKey {
        public Key key;
        public string[] symbols;
        
        public static HotKey[] HotKeys = {
            new HotKey {
                key = Key.a,
                symbols = new string[] { "ɑ", "æ", "ɐ", "ɑ̃" }
            }, new HotKey {
                key = Key.b,
                symbols = new string[] { "β", "ɓ", "ʙ" }
            }, new HotKey {
                key = Key.c,
                symbols = new string[] { "ç", "ɕ" }
            }, new HotKey {
                key = Key.d,
                symbols = new string[] { "ð", "d͡ʒ", "ɖ", "ɗ" }
            }, new HotKey {
                key = Key.e,
                symbols = new string[] { "ə", "ɚ", "ɵ", "ɘ" }
            }, new HotKey {
                key = Key.Key_3,
                symbols = new string[] { "ɛ", "ɜ", "ɝ", "ɛ̃", "ɞ" }
            }, new HotKey {
                key = Key.g,
                symbols = new string[] { "ɠ", "ɢ", "ʛ" }
            }, new HotKey {
                key = Key.h,
                symbols = new string[] { "ħ", "ɦ","ɥ", "ɧ", "ʜ" }
            }, new HotKey {
                key = Key.i,
                symbols = new string[] { "ɪ", "ɨ", "ɪ̈" }
            }, new HotKey {
                key = Key.j,
                symbols = new string[] { "ʝ", "ɟ", "ʄ" }
            }, new HotKey {
                key = Key.l,
                symbols = new string[] { "ɫ", "ɭ", "ɬ", "ʟ", "ɮ" }
            }, new HotKey {
                key = Key.m,
                symbols = new string[] { "ɱ" }
            }, new HotKey {
                key = Key.n,
                symbols = new string[] { "ŋ", "ɲ", "ɳ", "ɴ" }
            }, new HotKey {
                key = Key.o,
                symbols = new string[] { "ɔ","œ", "ɒ", "ɔ̃", "ɶ" }
            }, new HotKey {
                key = Key.Key_0,
                symbols = new string[] { "ø" }
            }, new HotKey {
                key = Key.p,
                symbols = new string[] { "ɸ" }
            }, new HotKey {
                key = Key.r,
                symbols = new string[] { "ɾ", "ɹ", "ʁ", "ʀ", "ɻ", "ɽ", "ɺ" }
            }, new HotKey {
                key = Key.s,
                symbols = new string[] { "ʃ", "ʂ" }
            }, new HotKey {
                key = Key.t,
                symbols = new string[] { "θ", "t͡ʃ", "t͡s", "ʈ" }
            }, new HotKey {
                key = Key.u,
                symbols = new string[] { "ʊ", "ʉ" }
            }, new HotKey {
                key = Key.v,
                symbols = new string[] { "ʌ", "ʋ", "ⱱ" }
            }, new HotKey {
                key = Key.w,
                symbols = new string[] { "ɯ", "ʍ", "ɰ" }
            }, new HotKey {
                key = Key.x,
                symbols = new string[] { "χ" }
            }, new HotKey {
                key = Key.y,
                symbols = new string[] { "ɣ", "ʎ", "ʏ", "ɤ" }
            }, new HotKey {
                key = Key.z,
                symbols = new string[] { "ʒ", "ʐ", "ʑ" }
            }, new HotKey {
                key = Key.Key_2,
                symbols = new string[] { "ʔ", "ʕ", "ʡ", "ʢ" }
            }, new HotKey {
                key = Key.q,
                symbols = new string[] { "ˈ", "ˌ" }
            }, new HotKey {
                key = Key.backslash,
                symbols = new string[] { "ǀ", "ǁ", "ǂ", "ǃ", "ʘ" }
            }, new HotKey {
                key = Key.colon,
                symbols = new string[] { ";" }
            }, new HotKey {
                key = Key.period,
                symbols = new string[] { "˩", "˨", "˧", "˦", "˥" }
            }, new HotKey {
                key = Key.grave,
                symbols = new string[] {
                    "ˤ", "ˀ", "ᵝ", "ᵊ", "ʱ", "ˡ", "ⁿ", "ʳ", "ᵗ", "ˠ", "ʼ"
                }
            },
        };
    }
}