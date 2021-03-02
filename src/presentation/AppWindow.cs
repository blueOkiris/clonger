/*
 * This will be the primary location for basic layout
 * There's four views in this application:
 *  - IPA keyboard
 *  - Document
 *  - Dictionary
 *  - Examples
 * Each of those has its own file and setup to keep code clean
 * This is just the primary window
 * Those will be widgets which can be placed in the main area of this window
 */

using System;
using Gtk;
using Clonger.Data;

namespace Clonger.Presentation {    
    class AppWindow : Window {
        public static readonly AppWindow Instance = new AppWindow();
        
        private Frame viewContainer;
        
        private AppWindow() : base(AppSettings.WindowTitle) {
            // Set up window properties
            SetSizeRequest(
                AppSettings.MinWindowSize.Item1,
                AppSettings.MinWindowSize.Item2
            );
            Resize(
                AppSettings.DefaultWindowSize.Item1,
                AppSettings.DefaultWindowSize.Item2
            );
            DeleteEvent += (object sender, DeleteEventArgs args) => {
                Application.Quit();
            };
            
            // Build application
            var mainContainer = new VBox(false, (int) AppSettings.Margin);
            
            var viewSwitchBar = new HBox(false, (int) AppSettings.Margin);
            var ipaButton = new Button("IPA");
            ipaButton.Clicked += (object sender, EventArgs args) => {
                switchViews(ViewType.IpaKeyboard);
            };
            viewSwitchBar.PackStart(
                ipaButton, false, false, AppSettings.Margin
            );
            var docButton = new Button("Document");
            docButton.Clicked += (object sender, EventArgs args) => {
                switchViews(ViewType.Document);
            };
            viewSwitchBar.PackStart(
                docButton, false, false, AppSettings.Margin
            );
            var dictButton = new Button("Dictionary");
            dictButton.Clicked += (object sender, EventArgs args) => {
                switchViews(ViewType.Dictionary);
            };
            viewSwitchBar.PackStart(
                dictButton, false, false, AppSettings.Margin
            );
            var exButton = new Button("Examples");
            exButton.Clicked += (object sender, EventArgs args) => {
                switchViews(ViewType.Examples);
            };
            viewSwitchBar.PackStart(
                exButton, false, false, AppSettings.Margin
            );
            viewSwitchBar.BorderWidth = AppSettings.BorderWidth;
            mainContainer.PackStart(
                viewSwitchBar, false, false, AppSettings.Margin
            );
            
            viewContainer = new Frame("IPA Typing Tool");
            mainContainer.PackStart(
                viewContainer, true, true, AppSettings.Margin
            );
            viewContainer.BorderWidth = AppSettings.BorderWidth;
            
            Add(mainContainer);
            switchViews(ViewType.IpaKeyboard);
        }
        
        private void switchViews(ViewType view) {
            switch(view) {
                case ViewType.IpaKeyboard:
                    viewContainer.Label = "IPA Typing Tool";
                    break;
                
                case ViewType.Document:
                    viewContainer.Label = "Language Documentation Manager";
                    break;
                
                case ViewType.Dictionary:
                    viewContainer.Label = "Dictionary Storage System";
                    break;
                
                case ViewType.Examples:
                    viewContainer.Label = "List of Examples";
                    break;
            }
        }
    }
}