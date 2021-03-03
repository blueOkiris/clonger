/*
 * This handles the logic for key presses
 * and keeping the text buffer properly updated,
 * (i.e. the funtionality)
 * for the IPA view
 */

using System.Linq;
using Gdk;
using Clonger.Data;

namespace Clonger.Domain {
    class IpaTextManager {
        public string Input;
        
        private bool isAltHeld, altReset;
        private Key lastKey;
        private int altKeyIndex;
        
        public IpaTextManager() {
            isAltHeld = false;
            altKeyIndex = 0;
            lastKey = Key.a;
            altReset = true;
            Input = "";
        }
        
        // Intercept keys to do alt key replacements
        public void OnKeyPress(Key key) {
            /*Console.WriteLine(
                "Key pressed: {0}, Alt: {1}", key, isAltHeld
            );*/
            if(key == Key.Alt_L) {
                isAltHeld = true;
                altReset = true;
            } else if(isAltHeld) {
                // Reset on key change
                if(altReset || key != lastKey) {
                    lastKey = key;
                    altKeyIndex = 0;
                    altReset = false;
                } else if(key == lastKey) {
                    var hotKeys = HotKey.HotKeys.Where(
                        (HotKey hotKey) => hotKey.key == key
                    ).ToArray();
                    if(hotKeys.Length > 0) {
                        if(altKeyIndex == 0) {
                            var lastSymbLen = hotKeys[0].symbols[
                                hotKeys[0].symbols.Length - 1
                            ].Length;
                            Input = Input.Remove(
                                Input.Length - lastSymbLen, lastSymbLen
                            );
                        } else {
                            var lastSymbLen = hotKeys[0].symbols[
                                altKeyIndex - 1
                            ].Length;
                            Input = Input.Remove(
                                Input.Length - lastSymbLen, lastSymbLen
                            );
                        }
                    }
                }
                
                foreach(var hotKey in HotKey.HotKeys) {
                    if(key == hotKey.key) {
                        Input += hotKey.symbols[altKeyIndex++];
                        if(altKeyIndex >= hotKey.symbols.Length) {
                            altKeyIndex = 0;
                        }
                        break;
                    }
                }
            }
        }
        
        // Reset alt-key
        public void OnKeyRelease(Key key) {
            /*Console.WriteLine(
                "Key released: {0}, {1}", key, isAltHeld
            );*/
            if(key == Key.Alt_L) {
                isAltHeld = false;
                altReset = true;
                altKeyIndex = 0;
            }
        }
        
        public static string hotKeyStr(HotKey hotKey) {
            switch(hotKey.key) {
                default:
                    return hotKey.key.ToString();
                case Gdk.Key.grave:
                    return "`";
                case Gdk.Key.backslash:
                    return "\\";
                case Gdk.Key.period:
                    return ".";
                case Gdk.Key.colon:
                    return ":";
                case Gdk.Key.Key_0:
                    return "0";
                case Gdk.Key.Key_1:
                    return "1";
                case Gdk.Key.Key_2:
                    return "2";
                case Gdk.Key.Key_3:
                    return "3";
                case Gdk.Key.Key_4:
                    return "4";
                case Gdk.Key.Key_5:
                    return "5";
                case Gdk.Key.Key_6:
                    return "6";
                case Gdk.Key.Key_7:
                    return "7";
                case Gdk.Key.Key_8:
                    return "8";
                case Gdk.Key.Key_9:
                    return "9";
            }
        }
    }
}
