using System;
using Gtk;
using Clonger.Data;

namespace Clonger.Presentation.Views {
    class IpaKeyboardView : VBox {
        public IpaKeyboardView() : base(false, (int) AppSettings.Margin) {
            
        }
    }
}