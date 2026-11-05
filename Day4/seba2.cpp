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
    vector<vector<int>> vettore_numero_vicini(ncolonne, vector<int>(nrighe));
    int risultato = 0;
    for (int r = 0; r < nrighe; r++)
    {
        for (int c = 0; c < ncolonne; c++)
        {
            int vicini;
            if (griglia[c][r] != '@')
            {
                vicini = -1000;
            }
            else
            {
                vicini = 0;

                for (int f = 0; f <= 7; f++)
                {
                    int cdx = c + deltax[f];
                    int rdy = r + deltay[f];
                    if (cdx >= 0 && ncolonne > cdx && rdy >= 0 && nrighe > rdy && griglia[cdx][rdy] == '@')
                    {
                        vicini++;
                    }
                }
            }
            vettore_numero_vicini[r][c] = (vicini);
        }
    }
    int k = 0;
    int j = 0;
    while (k == j)
    {
        k++;
        for (int r = 0; r < nrighe; r++)
        {
            for (int c = 0; c < ncolonne; c++)
            {
                if (vettore_numero_vicini[c][r] > (-500) && vettore_numero_vicini[c][r] < 4)
                {
                    vettore_numero_vicini[c][r] = -1000;

                    risultato++;
                    for (int f = 0; f <= 7; f++)
                    {
                        int cdx = c + deltax[f];
                        int rdy = r + deltay[f];
                        if (cdx >= 0 && ncolonne > cdx && rdy >= 0 && nrighe > rdy)
                        {
                            vettore_numero_vicini[cdx][rdy]--;

                            k = j;
                        }
                    }
                }
            }
        }
    }
    cout << risultato << endl;
}