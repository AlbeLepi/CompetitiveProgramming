#include <iostream>
#include <vector>

using namespace std;

int main()
{
    int n, m, k;
    cin >> n >> m >> k;
    vector<int> *friends = new vector<int>[n];
    for(int i = 0; i < m; i++) {
        int a, b;   cin >> a >> b;
        friends[a-1].push_back(b-1);
        friends[b-1].push_back(a-1);
    }
    int *common = new int[n*n];
    int count = 0;
    for(int i = 0; i < n*n; i++) common[i] = 0;
    for(int i = 0; i < n; i++) {
        for(int j = 0; j < friends[i].size(); j++) {
            for(int l = j+1; l < friends[i].size(); l++) {
                common[friends[i][j]*n+friends[i][l]]++;
                common[friends[i][l]*n+friends[i][j]]++;
                if(common[friends[i][j]*n+friends[i][l]] > count) count++;
                if(count >= k) {cout << "YES"; return 0;}
            }
        }
    }
    cout << "NO";
    return 0;
}
