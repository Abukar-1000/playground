using System;
using System.Net;
using System.Text.Json;


namespace baseC
{
    public class JsonResponse
    {
        public JsonResponse()
        {

        }
    }
    public class Program
    {
        public static void Main(string[] args) 
        {
            string strJson = Program.makeRequest("https://stormy-jumper.cyclic.app/F99");
            dynamic jsonResponse = JObject.Parse("{number:1000, str:'string', array: [1,2,3,4,5,6]}");;
            Console.WriteLine("Hi there");
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
            Console.WriteLine("data was " + result);
            return result;
        }
    }
}