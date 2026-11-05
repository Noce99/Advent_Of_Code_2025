#include <iostream>
#include <fstream>
#include <vector>

using namespace std;

int main()
{
    int risultato = 0;
    ifstream inp("input.txt");
    vector<string> range;
    string code;
    vector<string> cibi;
    string alimento;

    while (getline(inp, code))
    {
        if (code.empty())
        {
            break;
        }
        else
        {
            range.push_back(code);
        }
    }
    while (getline(inp, alimento))
    {
        cibi.push_back(alimento);
    }

    for (int i = 0; i < cibi.size(); i++)
    {
        long int cibi_i = stol(cibi[i]);
        for (int j = 0; j < range.size(); j++)
        {
            int posizione = range[j].find('-');
            long int inizioj = stol(range[j].substr(0, posizione));
            long int finej = stol(range[j].substr(posizione + 1));
            if (inizioj <= cibi_i && cibi_i <= finej)
            {
                risultato++;
                break;
            }
        }
    }

    cout << cibi[0] << endl;
    cout << range[0] << endl;
    cout << risultato << endl;
}