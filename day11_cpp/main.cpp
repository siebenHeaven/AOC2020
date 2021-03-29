#include <iostream>
#include <vector>
#include <unistd.h>

using namespace std;

int main() {
    vector<vector<char>> grids[2];
    int choose = 0;

    while(cin) {
        vector<char> temp;
        while(cin && cin.peek() != '\n') {
            char c;
            cin >> c;
            temp.push_back(c);
        }

        if(!cin)
            break;

        grids[0].push_back(temp);
        grids[1].push_back(temp);

        cin.ignore();
    }

    bool changed = true;
    auto rows = grids[0].size();
    auto cols = grids[0][0].size();
    grids[1] = grids[0];

    while(changed) {
        std::system("clear");
        changed = false;
        vector<vector<char>>& curr = grids[0];
        vector<vector<char>>& other = grids[1];

        for(int i=0; i<rows; i++) {
            for(int j=0; j<cols; j++) {
                if(curr[i][j] == 'L') {
                    bool occupied = false;
                    for(int y=-1; y<=1; y++) {
                        for(int x=-1; x<=1; x++) {
                            if((y == 0) && (x == 0))
                                continue;
                            int row = i, col = j;
                            do {
                                row += y;
                                col += x;
                            } while(((col > -1) && (col < cols)) && ((row > -1) && (row < rows)) && (curr[row][col] == '.'));
                            if (((col > -1) && (col < cols)) && ((row > -1) && (row < rows)) && (curr[row][col] == '#')) {
                                occupied = true;
                                break;
                            }
                        }
                        if(occupied)
                            break;
                    }
                    if(!occupied) {
                        other[i][j] = '#';
                        changed = true;
                    }
                } else if(curr[i][j] == '#') {
                    auto occupied_count = 0;
                    for(int y=-1; y<=1; y++) {
                        for(int x=-1; x<=1; x++) {
                            if((y == 0) && (x == 0))
                                continue;
                            int row = i, col = j;
                            do {
                                row += y;
                                col += x;
                            } while(((col > -1) && (col < cols)) && ((row > -1) && (row < rows)) && (curr[row][col] == '.'));
                            if (((col > -1) && (col < cols)) && ((row > -1) && (row < rows)) && (curr[row][col] == '#')) {
                                occupied_count++;
                            }
                        }
                    }
                    if(occupied_count >= 5) {
                        other[i][j] = 'L';
                        changed = true;
                    }
                }
                cout << other[i][j];
            }
            cout << '\n';
        }
        cout << endl;
        grids[0] = grids[1];
        usleep(10000);
    };

    auto occupied = 0;
    auto& curr = grids[0];
    for(int i=0; i<rows; i++) {
        for(int j=0; j<cols; j++) {
            occupied += (curr[i][j] == '#')?1:0;
        }
    }

    cout << "Occupied: " << occupied << endl;

    return 0;
}
