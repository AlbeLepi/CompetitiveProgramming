#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

struct point{int x, y, dist;};

int search(int a, int inizio, int fine, vector<point> &lista){
    if(inizio == fine) return inizio;
    int medio = (inizio+fine)/2;
    if(a <= lista[medio].dist) return search(a, inizio, medio, lista);
    else return search(a, medio+1, fine, lista);
}

int main() {
    int m;  cin >> m;
    vector<point> booth;
    point a; a.x = 0;
    for(int i = 0; i < m; i++) {
        cin >> a.y;
        booth.push_back(a);
    }
    sort(booth.begin(), booth.end(), [](point &a, point &b){return a.y < b.y;});
    point sam;  cin >> sam.x >> sam.y;
    int n;  cin >> n;
    vector<point> enemies;
    for(int i = 0; i < n; i++) {
        cin >> a.x >> a.y;
        enemies.push_back(a);
    }
    sort(enemies.begin(), enemies.end(), [](point &a, point &b){return a.y < b.y;});
    vector<point> *lists = new vector<point>[m];
    int ind = 0;
    for(int i = 0; i < n; i++) {
        if(ind < m-1 && enemies[i].y-booth[ind].y > booth[ind+1].y-enemies[i].y) ind++;
        if(enemies[i].y-booth[ind].y > 0) enemies[i].dist = enemies[i].y-booth[ind].y+enemies[i].x; 
        else enemies[i].dist = -enemies[i].y+booth[ind].y+enemies[i].x;
        lists[ind].push_back(enemies[i]);
    }
    for(int i = 0; i < m; i++) sort(lists[ind].begin(), lists[ind].end(), [](point &a, point &b){return a.dist < b.dist;});
    int minim = n;
    for(int ind = 0; ind < m; ind++) {
        if(sam.y-booth[ind].y > 0) sam.dist = sam.y-booth[ind].y+sam.x; 
        else sam.dist = -sam.y+booth[ind].y+sam.x;
        int a = search(sam.dist, 0, lists[ind].size(), lists[ind]);
        if(a < minim) minim = a;
    }
    cout << minim << endl;
    return 0;
}

