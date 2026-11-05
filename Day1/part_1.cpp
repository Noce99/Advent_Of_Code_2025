#include <iostream>
#include <fstream>
#include <string>
#include <vector>


int main(){
    std::ifstream file("input.txt");

    std::vector<std::vector<int>> data;

    std::string line;
    while (std::getline(file, line)) {
        int first_int;
        if (line[0] == 'L'){
            first_int = 0;
        }else{
            first_int = 1;
        }
        std::string string_second_int(line.begin() + 1, line.end());
        int second_int = std::stoi(string_second_int);

        //std::cout << "[" << first_int << "]" << " " << second_int << "\n";
        std::vector<int> element;
        element.push_back(first_int);
        element.push_back(second_int);

        data.push_back(element);
    }
    file.close();

    int c_1 = 50;
    int c_2 = 0;

    for (int i=0; i<data.size(); i++){
        if (data[i][0] == 0){
            c_1 -= data[i][1];
        }else{
            c_1 += data[i][1];
        }
        if (c_1 % 100 == 0){
            c_2++;
        }
    }

    std::cout << c_2 << std::endl;
}