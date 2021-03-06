/*
 * This is the IPA typing tool
 *
 * It contains a box for inputting characters.
 * You can either input plain characters
 * or use the alt key like a T9 phone for special characters
 *
 * The hotkeys are shown above in a frame
 */

using System;
using System.Text;
using System.Linq;
using Gtk;
using Clonger.Data;
using Clonger.Domain;

namespace Clonger.Presentation.Views {
    class IpaKeyboardView : VBox {
        private IpaTextManager textManager;
        
        public IpaKeyboardView() : base(false, (int) AppSettings.Margin) {
            textManager = new IpaTextManager();
            
            createHotKeyTable();
            addTextField();
        }
        
        private void addTextField() {
            var textFieldScroll = new ScrolledWindow();
            var textView = new TextView();
            textView.KeyPressEvent +=
                (object sender, KeyPressEventArgs args) => {
                    textManager.Input = (sender as TextView).Buffer.Text;
                    textManager.OnKeyPress(args.Event.Key);
                    (sender as TextView).Buffer.Text = textManager.Input;
                };
            textView.KeyReleaseEvent += 
                (object sender, KeyReleaseEventArgs args) => {
                    textManager.OnKeyRelease(args.Event.Key);
                };
            textFieldScroll.Add(textView);
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
                hotKeyLabelFrame.Add(
                    new Label(IpaTextManager.hotKeyStr(hotKey))
                );
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
    }
}