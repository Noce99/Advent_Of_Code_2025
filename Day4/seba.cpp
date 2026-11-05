#include <iostream>
#include <fstream>
#include <vector>

using namespace std;

int main()
{
    vector<string> griglia;
    ifstream inp("input.txt");
    string riga;
    vector<int> deltax = {1, 1, 1, 0, 0, -1, -1, -1};
    vector<int> deltay = {1, 0, -1, 1, -1, 1, 0, -1};

    while (getline(inp, riga))
        griglia.push_back(riga);
    int nrighe = griglia.size();
    int ncolonne = griglia[0].size();
    cout << nrighe << endl;
    cout << ncolonne << endl;

    int risultato = 0;
    for (int r = 0; r < nrighe; r++)
    {
        cout<<"nuova clonna"<<endl;
        for (int c = 0; c < ncolonne; c++)
        {
            if (griglia[c][r] == '@')
            {
                int vicini = 0;

                for (int f = 0; f <= 7; f++)
                {
                    int cdx = c + deltax[f];
                    int rdy = r + deltay[f];
                    if (cdx >= 0 && ncolonne > cdx && rdy >= 0 && nrighe > rdy && griglia[cdx][rdy] == '@')
                    {
                        vicini++;
                    }
                }
                cout << vicini << endl;
                if (vicini < 4)
                {
                    risultato++;
                }
            }
        }
    }
    cout << risultato << endl;
}