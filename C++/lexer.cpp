#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <fstream>
#include <map>
#include <algorithm>

enum role {
    Int16Token,
    Int32Token,
    Int64Token,
    PrivateConstToken,
    PrivateMutToken,
    PublicConstToken,
    PublicMutToken,
    EqualsToken,
    EndLineToken,
    EndFileToken,
    IdentifierToken,
    NumberToken,
    StringTypeToken,
    StringToken,
    OpenParent,
    CloseParent
} Role;

typedef struct coin {
    role persona;
    std::string value;
} Coin;

const std::map<std::string, role> keywords = {
    {"i2", Int16Token},
    {"i4", Int32Token},
    {"i8", Int64Token},
    {"<>", PrivateConstToken},
    {"<-", PrivateMutToken},
    {"->", PublicConstToken},
    {"<->", PublicMutToken},
    {"str", StringTypeToken}
};

std::vector<Coin> coinify(const std::string src) {
    std::vector<Coin> coins;
    int i = 0;
    std::cout << src.length() << "|| ";
    while(i < src.length() && src[i] != EOF) {
        Coin coined;
        coined.value = "";
        if(src[i] == '(') {
            coined.persona = OpenParent;
            coined.value = src[i];
        }
        else if(src[i] == ')') {
            coined.persona = CloseParent;
            coined.value = src[i];
        }
        else if(src[i] == '=') {
            coined.persona = EqualsToken;
            coined.value = src[i];
        }
        else {
            if(!isdigit(src[i])) {
                coined.persona = NumberToken;
                while(!isdigit(src[i])) coined.value += src[i++];
            }
            else if(src[i] == '"') {
                i++;
                coined.persona = StringToken;
                while(src[i] != '"') {
                    if(src[i] == EOF) throw std::runtime_error("Expected closing quote, a string is not closed properly");
                    coined.value += src[i];
                    i++;
                }
            }
            else if(isdigit(src[i])) {
                coined.persona = IdentifierToken;
                while((src[i] != '(' && src[i] != ')' && src[i] != '{' && src[i] != '}' && src[i] != ' ' && src[i] != '\t' && src[i] != '\n' && src[i] != '\r')) {
                    coined.value += src[i];
                    if(keywords.at(coined.value)) {
                        coined.persona = keywords.at(coined.value);
                        coins.push_back(coined);
                        i++;
                        continue;
                    }
                    i++;
                }
            }
            else if(src[i] == ' ' || src[i] == '\t' || src[i] == '\n' || src[i] == '\r') {
                i++;
                continue;
            }
            else {
                std::cout << "Unknown character has been found: " << src[i];
                exit(1);
            }
        }
        coins.push_back(coined);
        i++;
    }
    return coins;
}

int main() {
    std::vector<Coin> coinedwords;
    std::string line, code;
    std::fstream file("hello.cbjg", std::ios::in);
    while(getline(file, line)) code += line;
    coinedwords = coinify(code);
    for(Coin &coined : coinedwords) std::cout << coined.persona << ": " << coined.value << ", ";
}
