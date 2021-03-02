using System;
using Gtk;
using Clonger.Data;

namespace Clonger.Presentation.Views {
    class ExampleListView : VBox {
        public ExampleListView() : base(false, (int) AppSettings.Margin) {
            
        }
    }
}