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
    }
}
