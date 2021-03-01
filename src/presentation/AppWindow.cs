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
using Gdk;
using Clonger.Data;

namespace Clonger.Presentation {
    class AppWindow : Gtk.Window {
        public static readonly AppWindow Instance = new AppWindow();
        
        private AppWindow() : base(AppSettings.WindowTitle) {
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
        }
    }
}