using System;
using Gtk;
using Clonger.Data;

namespace Clonger.Presentation {
    class Program {
        public static void Main() {
            Console.WriteLine("Welcome to clonger!");
            Application.Init();
            AppWindow.Instance.ShowAll();
            Application.Run();
        }
    }
}
