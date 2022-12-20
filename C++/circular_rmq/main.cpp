#include <iostream>
#include <vector>
#include <string>
using namespace std;
 
struct duo {int a,b;};
 
vector<long long int> nums;
vector<long long int> st;
vector<duo> interval;
 
void iniz(int ind, int n, int poszero) {
    if(2*ind+2 >= interval.size()) {
        int pos;    if(ind >= poszero) pos = ind-poszero; else pos = ind-poszero+n;
        interval[ind].a = pos;  interval[ind].b = pos;
        st[ind] = nums[pos];
    } else {
        iniz(2*ind+1, n, poszero);  iniz(2*ind+2, n, poszero);
        interval[ind].a = interval[2*ind+1].a;  interval[ind].b = interval[2*ind+2].b;
        if(st[2*ind+1] > st[2*ind+2]) st[ind] = st[2*ind+2]; else st[ind] = st[2*ind+1];
    }
    return;
}
 
void inc(int ind, int i, int j, long long int d) {
    if(d == 0) return;
    else if(ind >= st.size() || interval[ind].a > j || interval[ind].b < i) return;
    else if(interval[ind].a >= i && interval[ind].b <= j) st[ind] += d; //lazy propagation
    else {
        //false minimum because of lazy propagation:
        long long int fm; if(st[2*ind+1] > st[2*ind+2]) fm = st[2*ind+2]; else fm = st[2*ind+1];
        inc(2*ind+1, i, j, d); inc(2*ind+2, i, j, d);
        //correct minimum:
        if(st[2*ind+1] > st[2*ind+2]) st[ind] = st[ind] - fm + st[2*ind+2];
        else st[ind] = st[ind] - fm + st[2*ind+1];
    }
    return;
}
 
long long int rmq(int ind, int i, int j, long long int def) {
    if(ind >= st.size() || interval[ind].a > j || interval[ind].b < i) return def+1;
    else if(interval[ind].a >= i && interval[ind].b <= j) return st[ind];
    else {
        //false minimum
        long long int fm; if(st[2*ind+1] > st[2*ind+2]) fm = st[2*ind+2]; else fm = st[2*ind+1];
        //actual minimum
        long long int a = rmq(2*ind+1, i, j, def) + st[ind] - fm;
        long long int b = rmq(2*ind+2, i, j, def) + st[ind] - fm;
        if(a > b) return b; else return a;
    }
}
 
int main() {
    int n;  cin >> n;
    for(int i = 0; i < n; i++){long long int a; cin >> a;  nums.push_back(a);}
    long long int def = 0;    for(int i = 0; i < n; i++) if(nums[i] > def) def = nums[i];
    int m;  cin >> m; getchar(); //because of dirty input reading
    for(int i = 0; i < 2*n-1; i++) st.push_back(0);
    int poszero = 0;    while(2*poszero+1 < 2*n-1) poszero = 2*poszero+1;
    duo c;  c.a = 0; c.b = 0;
    for(int i = 0; i < 2*n-1; i++) interval.push_back(c);
    iniz(0, n, poszero);
    for (int i = 0; i < m; i++) {
        //veeery dirty input reading
        string riga;    getline(cin, riga);
        long long int line[3], s, sig; line[0]=0;  line[1]=0;  line[2]=0;  s=0; sig=1;
        for(int j = 0; j < riga.size(); j++){
            if(riga[j] == ' ') s++;
            else if(riga[j] == '-') sig = -1;
            else line[s] = line[s]*10 + riga[j] - '0';
        }
        if(sig == -1) line[2] = -line[2];
        //here we go
        if(s == 1) {
            long long int v;
            if(line[0] <= line[1]) v = rmq(0, line[0], line[1], def);
            else {
                long long int a = rmq(0, 0, line[1], def);
                long long int b = rmq(0, line[0], n-1, def);
                if(a < b) v = a; else v = b;
            }
            cout << v << endl;
        } else {
            if (line[0] <= line[1]) {inc(0, line[0], line[1], line[2]);}
            else {
                inc(0, 0, line[1], line[2]);
                inc(0, line[0], n-1, line[2]);
            }
            if(line[2] > 0) def += line[2];
        }
    }
    return 0;
}