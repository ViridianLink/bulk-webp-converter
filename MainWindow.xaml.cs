using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Windows;
using System.Windows.Forms;


namespace Webp_Converter
{
    public partial class MainWindow : Window
    {
        private readonly string rootPath;
        private string selectedFolder;
        private List<string> outputLog = new List<string>();

        public MainWindow()
        {
            InitializeComponent();

            rootPath = Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData), "Oscar Six", "WebpConverter");

            addLog("Quiting the converter while it's working can cause the current working file to\n be corrupt");
            addLog("Make sure to back up any images before converting");
        }

        private void addLog(string text)
        {
            outputLog.Insert(0, text);
            WindowOutput.Text = string.Join("\n", outputLog);
        }

        private void SelectFolder_Click(object sender, RoutedEventArgs e)
        {
            using (FolderBrowserDialog dialog = new())
            {
                dialog.RootFolder = Environment.SpecialFolder.Recent;

                if (dialog.ShowDialog() == System.Windows.Forms.DialogResult.OK)
                {
                    selectedFolder = dialog.SelectedPath;
                    ChosenFolder.Text = $"Selected File: {selectedFolder}";
                }
            }
        }

        private void StartConversion_Click(object sender, RoutedEventArgs e)
        {
            SearchOption searchOptions = SearchOption.TopDirectoryOnly;
            if (Setting_SubFolders.IsChecked == true) { searchOptions = SearchOption.AllDirectories; }

            List<string> files = new();
            if (Setting_jpg.IsChecked == true) { files.AddRange(Directory.GetFiles(selectedFolder, "*.jpg", searchOptions)); }
            if (Setting_png.IsChecked == true) { files.AddRange(Directory.GetFiles(selectedFolder, "*.png", searchOptions)); }
            if (Setting_webp.IsChecked == true) { files.AddRange(Directory.GetFiles(selectedFolder, "*.webp", searchOptions)); }

            foreach (string file in files)
            {
                Process process = new();
                ProcessStartInfo startInfo = new();
                startInfo.FileName = "cmd.exe";
                startInfo.WorkingDirectory = rootPath;
                startInfo.Arguments = $"/C .\\cwebp.exe /b -mt -progress -m 6 -q {Math.Round(QualitySlider.Value)} \"{file}\" -o \"{Path.ChangeExtension(file, ".webp")}\"";
                process.StartInfo = startInfo;

                process.Start();
                process.WaitForExit();
                if (!file.EndsWith("webp")) { File.Delete(file); }

                addLog($"{file} has finished processing");
            }
            addLog("ALL FILES HAVE BEEN COMPLETED!");
        }
    }
}
