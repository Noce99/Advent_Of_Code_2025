#include <iostream>
#include <fstream>
#include <vector>
#include <sstream>

using namespace std;

int main()
{
    long int risultato = 0;
    vector<char> segni;
    vector<vector<int>> griglia;
    ifstream inp("input.txt");
    string caso;
    int righe = 0;
    while (getline(inp, caso))
    {
        if (caso[0] == '*' || caso[0] == '+')
        {
            for (int i = 0; i < caso.size(); i++)
            {
                if (caso[i] != ' ')
                {
                    segni.push_back(caso[i]);
                }
            }
        }
        else
        {
            stringstream numeri;
            griglia.push_back(vector<int>());
            for (int i = 0; i < caso.size(); i++)
            {

                if (caso[i] != ' ')
                {
                    numeri << caso[i];
                }
                else
                {
                    if (numeri.str() != "")
                    {
                        cout << numeri.str() << endl;
                        griglia[righe].push_back(stol(numeri.str()));

                        numeri = stringstream();
                    }
                }
            }
            if (numeri.str() != "")
            {
                griglia[righe].push_back(stol(numeri.str()));
            }
            righe++;
        }
    }

    for (int i = 0; i < segni.size(); i++)
    {
        long int parziale = 1;
        for (int j = 0; j < righe; j++)
        {
            if (segni[i] == '+')
            {
                risultato += griglia[j][i];
            }
            else
            {

                parziale *= griglia[j][i];
            }
        }
        risultato += parziale;
    }
    cout<<risultato<<endl;
}