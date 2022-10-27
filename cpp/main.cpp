#include<iostream>
#include <sstream>
#include <fstream>
/*creating work around //easier way then creating multiple get line also converting variable to anythihg i want!! boom baby */


template <typename INPUT, typename OUTPUT> 
OUTPUT convert (INPUT content){ //take in content turn to float
    OUTPUT result; //output 
    std: :stringstream ss; //input file ss
    ss << content; //
    ss >> result;
    return result;
}

 /*-----------------*/

 
float convertLine(std::string line){
    std::string strNum = "";
    std::string strNum = "";
    float result = 0;
    std::stringstream ss;
    ss.str(line);

    while(getline(ss, strNum, ',')) 
    {
        result += convert<std::string, float>(strNum);
    }
    if (result < 0){ //creating results less than zero question 
        result = 0;
    }

    return result; //return zero or other =0
}

/* main function starting */

int main() {

    //std:: string myStr = convert<int, std: :string>()
    std::ifstream input_file("input.txt");
    std::string currentLine;
    if (input_file.is_open() && output_file.is_open()){
        while (getline(input_file,currentLine))
        { 
            //conver input to float
        float times = convertLine(currentLine); //counting the times to output (dont need a for loop)
        
        getline(input_file,currentLine)
            for(float i = 0; i < times; i++)
            {
                // put values into output file
            }
            
        // istream(currentLine); // reading
        // int integers[3]; //declaring how many we want to read in 
        // char delimiter; // we create the delimiter
        // istream ›› integers[0];

        for (int i = 1; i < 3; ++i)
        {
            istream >> delimeter >> integers[i];
            std:: cout << convertLine(currentLine) << "\n";
            float f1Num = convert<int, float> (integer[i]); //what if we have float question 
        }

        }
    }
    else {
        std::cout << "file is not open\n";
    }

    /*----close the file---*/
    input_file.close(); // cleanly closes the input file
    output_file.close(); // cleanly closes the output file
    return 0;
}
// returns


/*end of code*/