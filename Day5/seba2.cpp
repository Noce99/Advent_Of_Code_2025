#include <iostream>
#include <fstream>
#include <vector>

using namespace std;

int main()
{
    long int risultato = 0;
    ifstream inp("input.txt");
    vector<string> range;
    string code;
    vector<string> cibi;
    string alimento;
    vector<long int> vettore_inizi_ordinati;
    vector<long int> vettore_fini_ordinate;
    while (getline(inp, code))
    {
        if (code.empty())
        {
            break;
        }
        range.push_back(code);
        int meta = code.find('-');
        string inizio = code.substr(0, meta);
        string fine = code.substr(meta + 1);
        long int intinizio = stol(inizio);
        long int intfine = stol(fine);

        int i = vettore_fini_ordinate.size();
        for (int j = 0; j < vettore_fini_ordinate.size(); j++)
        {
            if (intfine < vettore_fini_ordinate[j])
            {
                i = j;
                break;
            }
            if (intfine == vettore_fini_ordinate[j])
            {
                if (intinizio < vettore_inizi_ordinati[j])
                {
                    i = j;
                    break;
                }
            }
        }

        vettore_fini_ordinate.insert(vettore_fini_ordinate.begin() + i, intfine);
        vettore_inizi_ordinati.insert(vettore_inizi_ordinati.begin() + i, intinizio);
    }
    int sizef = vettore_fini_ordinate.size();
    for (int k = 0; k < sizef; k++)
    {
        cout << vettore_fini_ordinate[k] << "-" << vettore_inizi_ordinati[k] << endl;
    }
    long int nuovo_inizio = vettore_inizi_ordinati[0];

    for (int counter = 0; counter < sizef - 1; counter++)
    {
        cout << "nuovo inizio pre=" << nuovo_inizio << endl;
        long int nuova_fine = vettore_fini_ordinate[counter];
        cout << "nuova fine pre=" << nuova_fine << endl;
        if (vettore_inizi_ordinati[counter + 1] <= nuova_fine)
        {
            cout << "in primo if \n";
            if (vettore_inizi_ordinati[counter + 1] < nuovo_inizio)
            {
                nuovo_inizio = vettore_inizi_ordinati[counter + 1];
            }
            // else{
            //  nuovo_inizio=
            //  }
        }
        else
        {
            cout << "else" << endl;
            risultato += nuova_fine - nuovo_inizio + 1;
            nuovo_inizio = vettore_inizi_ordinati[counter + 1];
        }

        cout << risultato << endl;
    }
    cout << nuovo_inizio << endl;
    risultato += vettore_fini_ordinate[sizef - 1] - nuovo_inizio + 1;
    // risultato+= (vettore_fini_ordinate[sizef-1]-nuovo_inizio);
    cout << risultato << endl;
}