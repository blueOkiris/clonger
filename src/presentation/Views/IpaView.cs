using System;
using System.Text;
using System.Linq;
using Gtk;
using Clonger.Data;

namespace Clonger.Presentation.Views {
    class IpaKeyboardView : VBox {
        private bool isAltHeld, altReset;
        private Gdk.Key lastKey;
        private int altKeyIndex;
        private TextView input;
        
        public IpaKeyboardView() : base(false, (int) AppSettings.Margin) {
            isAltHeld = false;
            altKeyIndex = 0;
            lastKey = Gdk.Key.a;
            altReset = true;
            
            createHotKeyTable();
            addTextField();
        }
        
        // Intercept keys to do alt key replacements
        private void onKeyPress(object sender, KeyPressEventArgs args) {
            /*Console.WriteLine(
                "Key pressed: {0}, Alt: {1}", args.Event.Key, isAltHeld
            );*/
            if(args.Event.Key == Gdk.Key.Alt_L) {
                isAltHeld = true;
                altReset = true;
            } else if(isAltHeld) {
                // Reset on key change
                if(altReset || args.Event.Key != lastKey) {
                    lastKey = args.Event.Key;
                    altKeyIndex = 0;
                    altReset = false;
                } else if(args.Event.Key == lastKey) {
                    var hotKeys = HotKey.HotKeys.Where(
                        (HotKey hotKey) => hotKey.key == args.Event.Key
                    ).ToArray();
                    if(hotKeys.Length > 0) {
                        var oldText = input.Buffer.Text;
                        if(altKeyIndex == 0) {
                            var lastSymbLen = hotKeys[0].symbols[
                                hotKeys[0].symbols.Length - 1
                            ].Length;
                            input.Buffer.Text = input.Buffer.Text.Remove(
                                oldText.Length - lastSymbLen, lastSymbLen
                            );
                        } else {
                            var lastSymbLen = hotKeys[0].symbols[
                                altKeyIndex - 1
                            ].Length;
                            input.Buffer.Text = input.Buffer.Text.Remove(
                                oldText.Length - lastSymbLen, lastSymbLen
                            );
                        }
                    }
                }
                
                foreach(var hotKey in HotKey.HotKeys) {
                    if(args.Event.Key == hotKey.key) {
                        input.Buffer.Text += hotKey.symbols[altKeyIndex++];
                        if(altKeyIndex >= hotKey.symbols.Length) {
                            altKeyIndex = 0;
                        }
                        break;
                    }
                }
            }
        }
        
        // Reset alt-key
        private void onKeyRelease(object sender, KeyReleaseEventArgs args) {
            /*Console.WriteLine(
                "Key released: {0}, {1}", args.Event.Key, isAltHeld
            );*/
            if(args.Event.Key == Gdk.Key.Alt_L) {
                isAltHeld = false;
                altReset = true;
                altKeyIndex = 0;
            }
        }
        
        private void addTextField() {
            var textFieldScroll = new ScrolledWindow();
            input = new TextView();
            input.KeyPressEvent += onKeyPress;
            input.KeyReleaseEvent += onKeyRelease;
            textFieldScroll.Add(input);
            PackStart(textFieldScroll, true, true, AppSettings.Margin);
        }
        
        private void createHotKeyTable() {
            var hotKeyView = new Frame("Hotkeys (Alt + key multiple times)");
            
            var hotKeyScroll = new ScrolledWindow();
            hotKeyScroll.VscrollbarPolicy = PolicyType.Never;
            hotKeyScroll.HscrollbarPolicy = PolicyType.Automatic;
            
            var hotKeyBoxContainer = new VBox(false, (int) AppSettings.Margin);
            HBox row = null;
            for(int i = 0; i < HotKey.HotKeys.Length; i++) {
                // Add new row if needed
                if(i % AppSettings.MaxHotKeysPerRow == 0) {
                    if(row != null) {
                        hotKeyBoxContainer.PackStart(
                            row, false, false, AppSettings.Margin
                        );
                    }
                    row = new HBox(false, (int) AppSettings.Margin);
                }
                
                // Generate box
                var hotKey = HotKey.HotKeys[i];
                var hotKeyContainer = new HBox(false, (int) AppSettings.Margin);
                var hotKeyLabelFrame = new Frame();
                hotKeyLabelFrame.Add(new Label(hotKeyStr(hotKey)));
                hotKeyContainer.PackStart(
                    hotKeyLabelFrame, false, false, AppSettings.Margin
                );
                
                // Get ipa stuff
                var symbolList = new StringBuilder(": { ");
                foreach(var symbol in hotKey.symbols) {
                    symbolList.Append(symbol).Append(", ");
                }
                symbolList.Remove(symbolList.Length - 2, 2);
                symbolList.Append(" }");
                
                hotKeyContainer.PackStart(
                    new Label(symbolList.ToString()), false, false,
                    AppSettings.Margin
                );
                
                row.PackStart(
                    hotKeyContainer, false, false, AppSettings.Margin
                );
            }
            hotKeyBoxContainer.PackStart(
                row, false, false, AppSettings.Margin
            );
            
            hotKeyScroll.Add(hotKeyBoxContainer);
            hotKeyView.Add(hotKeyScroll);
            PackStart(hotKeyView, false, false, AppSettings.Margin);
        }
        
        private static string hotKeyStr(HotKey hotKey) {
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