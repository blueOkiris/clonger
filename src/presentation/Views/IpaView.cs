using System;
using System.Text;
using Gtk;
using Clonger.Data;

namespace Clonger.Presentation.Views {
    class IpaKeyboardView : VBox {
        public IpaKeyboardView() : base(false, (int) AppSettings.Margin) {
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