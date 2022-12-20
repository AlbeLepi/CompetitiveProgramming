#include <iostream>
#include <vector>
#include <deque>

using namespace std;

struct fold{long long int st, en;};

long long int checklen(int n, vector<long long int> &l) {
    long long int *arr = new long long int[n];
    arr[0] = l[0];
    deque<fold> queue;
    long long int minfold = l[n-1];
    for(int i = 0; i < n; i++) {
        fold tr; tr.st = 0; tr.en = 0;
        while(queue.size() > 0 && queue.front().en <= l[i]) {
            //what's the best point where to fold before l[i] if Felix wants to fold in l[i]?
            tr = queue.front();
            queue.pop_front();
        }
        queue.push_front(tr);
        fold nw; nw.st = l[i]; nw.en = 2*l[i]-tr.st;
        if(nw.en > l[n-1]) {
            //is it completely folded?
            if(nw.en-nw.st < minfold) minfold = nw.en-nw.st;
        }
        else {
            while(queue.size() > 0 && queue.back().en > nw.en) queue.pop_back();
            //folding in those points is worse than folding in l[i]
            queue.push_back(nw);
        }
    }
    return minfold;
}

int main()
{
    int n;  cin >> n;
    vector<long long int> l;
    for(int i=0;i<n;i++) {
        long long int a;    cin >> a;
        //l[i] is the coordinate of the i-th point where Felix can fold
        if(i != 0) a += l[i-1];
        l.push_back(a);
    }
    cout << checklen(n, l);
    return 0;
}

