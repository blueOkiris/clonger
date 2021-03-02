using System;
using Gtk;
using Clonger.Data;

namespace Clonger.Presentation.Views {
    class LanguageDocumentationView : VBox {
        public LanguageDocumentationView() :
                base(false, (int) AppSettings.Margin) {
            
        }
    }
}