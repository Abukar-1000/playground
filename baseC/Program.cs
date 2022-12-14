using System;
using System.Net;
using Newtonsoft.Json;
using Newtonsoft.Json.Serialization;
using Json.Net;

namespace baseC
{
    public class JsonResponse
    {
        public string vitality = "";
        public string response = "";
    }

    public class Program
    {
        public static void Main(string[] args) 
        {
            string strJson = Program.makeRequest("https://stormy-jumper.cyclic.app/F99");
            JsonResponse product = new JsonResponse();
            JsonResponse deserializedProduct = JsonConvert.DeserializeObject<JsonResponse>(strJson);
            // dynamic jsonResponse = new JObject();
            Console.WriteLine("Vitality: " + deserializedProduct.vitality + "\nResponse: " + deserializedProduct.response);
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