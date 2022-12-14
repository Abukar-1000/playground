using System.Windows;
using System.Net;
using Newtonsoft.Json;
// using System.Windows.Forms;
using System.Runtime.InteropServices;
using System.Threading;
namespace baseC
{
    public class JsonResponse
    {
        public string vitality = "";
        public string response = "";
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct POINT
    {
        public int X;
        public int Y;
    }
    public class Program
    {
        public static POINT point;
        public static int xPos,yPos;

        public static void Main(string[] args) 
        {
            string strJson = Program.makeRequest("https://stormy-jumper.cyclic.app/F99");
            JsonResponse product = new JsonResponse();
            JsonResponse deserializedProduct = JsonConvert.DeserializeObject<JsonResponse>(strJson);
            // dynamic jsonResponse = new JObject();

            Console.WriteLine("Vitality: " + deserializedProduct.vitality + "\nResponse: " + deserializedProduct.response);
            
            while (true)
            {
                Program.GetPointPos();
            }
        }
        [DllImport("user32.dll")]
        static extern bool SetCursorPos(int X, int Y);


        [DllImport("user32.dll")]
        public static extern bool GetCursorPos(out POINT lpPoint);
        
        public static void GetPointPos()
        {
            Program.GetCursorPos(out Program.point);
            // Program.xPos = Program.point.X;
            // Program.yPos = Program.point.Y;
            Console.WriteLine($"({Program.point.X},{Program.point.Y})");
            Thread.Sleep(500);
        }
        public static string makeRequest(string url)
        {
            string result = "";
            WebRequest request = WebRequest.Create(url); 
            request.Method = "GET";
            WebResponse response = request.GetResponse();
            Stream stream = response.GetResponseStream();
            StreamReader reader = new StreamReader(stream);
            result = reader.ReadToEnd();
            return result;
        }
    }
}