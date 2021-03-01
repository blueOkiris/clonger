using System;
using Qml.Net;
using Qml.Net.Runtimes;
using clonger.data;

namespace clonger {
    namespace presentation {
        class Program {
            public static int Main(string[] args) {
                Console.WriteLine("Welcome to clonger!");
                
                RuntimeManager.DiscoverOrDownloadSuitableQtRuntime();
                var app = new QGuiApplication(args);
                var engine = new QQmlApplicationEngine();
                Qml.Net.Qml.RegisterType<QmlType>("test", 1, 1);
                engine.Load(AppSettings.QmlSrc + "Main.qml");
                return app.Exec();
            }
        }
    }
}
