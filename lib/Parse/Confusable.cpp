#include <algorithm>
#include <cctype>
#include <cstdint>
#include <string>
#include <string_view>
#include <unordered_map>
#include <unordered_set>
#include <vector>

namespace Shift {
namespace SEC {

struct ConfusableDiagnostic {
    std::string identifier;
    std::string otherIdentifier;
    std::string message;

    std::size_t line = 0;
    std::size_t column = 0;
};

class ConfusableChecker {
public:
    void check(
        std::string_view identifier,
        std::size_t line,
        std::size_t column
    ) {
        if (identifier.empty()) {
            return;
        }

        std::string skeleton = makeSkeleton(identifier);

        if (skeleton.empty()) {
            return;
        }

        auto found = skeletons.find(skeleton);

        if (found != skeletons.end()) {
            if (found->second != identifier) {
                ConfusableDiagnostic diagnostic;

                diagnostic.identifier =
                    std::string(identifier);

                diagnostic.otherIdentifier =
                    found->second;

                diagnostic.line = line;
                diagnostic.column = column;

                diagnostic.message =
                    "identifier '" +
                    std::string(identifier) +
                    "' is visually confusable with '" +
                    found->second +
                    "'";

                diagnostics.push_back(
                    std::move(diagnostic)
                );
            }
        } else {
            skeletons.emplace(
                std::move(skeleton),
                std::string(identifier)
            );
        }
    }

    const std::vector<ConfusableDiagnostic>&
    getDiagnostics() const {
        return diagnostics;
    }

    bool hasConfusables() const {
        return !diagnostics.empty();
    }

    void clear() {
        skeletons.clear();
        diagnostics.clear();
    }

    static bool containsNonASCII(
        std::string_view identifier
    ) {
        for (unsigned char c : identifier) {
            if (c >= 0x80) {
                return true;
            }
        }

        return false;
    }

private:
    std::unordered_map<
        std::string,
        std::string
    > skeletons;

    std::vector<ConfusableDiagnostic>
        diagnostics;

    static void appendUTF8(
        std::string& output,
        std::uint32_t codePoint
    ) {
        if (codePoint <= 0x7F) {
            output.push_back(
                static_cast<char>(codePoint)
            );
            return;
        }

        if (codePoint <= 0x7FF) {
            output.push_back(
                static_cast<char>(
                    0xC0 | (codePoint >> 6)
                )
            );

            output.push_back(
                static_cast<char>(
                    0x80 | (codePoint & 0x3F)
                )
            );

            return;
        }

        if (codePoint <= 0xFFFF) {
            output.push_back(
                static_cast<char>(
                    0xE0 | (codePoint >> 12)
                )
            );

            output.push_back(
                static_cast<char>(
                    0x80 |
                    ((codePoint >> 6) & 0x3F)
                )
            );

            output.push_back(
                static_cast<char>(
                    0x80 |
                    (codePoint & 0x3F)
                )
            );

            return;
        }

        output.push_back(
            static_cast<char>(
                0xF0 | (codePoint >> 18)
            )
        );

        output.push_back(
            static_cast<char>(
                0x80 |
                ((codePoint >> 12) & 0x3F)
            )
        );

        output.push_back(
            static_cast<char>(
                0x80 |
                ((codePoint >> 6) & 0x3F)
            )
        );

        output.push_back(
            static_cast<char>(
                0x80 |
                (codePoint & 0x3F)
            )
        );
    }

    static bool decodeUTF8(
        std::string_view text,
        std::size_t& index,
        std::uint32_t& codePoint
    ) {
        if (index >= text.size()) {
            return false;
        }

        const auto first =
            static_cast<unsigned char>(text[index]);

        if (first < 0x80) {
            codePoint = first;
            ++index;
            return true;
        }

        if ((first & 0xE0) == 0xC0) {
            if (index + 1 >= text.size()) {
                return false;
            }

            const auto second =
                static_cast<unsigned char>(
                    text[index + 1]
                );

            if ((second & 0xC0) != 0x80) {
                return false;
            }

            codePoint =
                ((first & 0x1F) << 6) |
                (second & 0x3F);

            index += 2;
            return true;
        }

        if ((first & 0xF0) == 0xE0) {
            if (index + 2 >= text.size()) {
                return false;
            }

            const auto second =
                static_cast<unsigned char>(
                    text[index + 1]
                );

            const auto third =
                static_cast<unsigned char>(
                    text[index + 2]
                );

            if ((second & 0xC0) != 0x80 ||
                (third & 0xC0) != 0x80) {
                return false;
            }

            codePoint =
                ((first & 0x0F) << 12) |
                ((second & 0x3F) << 6) |
                (third & 0x3F);

            index += 3;
            return true;
        }

        if ((first & 0xF8) == 0xF0) {
            if (index + 3 >= text.size()) {
                return false;
            }

            const auto second =
                static_cast<unsigned char>(
                    text[index + 1]
                );

            const auto third =
                static_cast<unsigned char>(
                    text[index + 2]
                );

            const auto fourth =
                static_cast<unsigned char>(
                    text[index + 3]
                );

            if ((second & 0xC0) != 0x80 ||
                (third & 0xC0) != 0x80 ||
                (fourth & 0xC0) != 0x80) {
                return false;
            }

            codePoint =
                ((first & 0x07) << 18) |
                ((second & 0x3F) << 12) |
                ((third & 0x3F) << 6) |
                (fourth & 0x3F);

            index += 4;
            return true;
        }

        return false;
    }

    static std::string mapCodePoint(
        std::uint32_t codePoint
    ) {
        switch (codePoint) {

            // Latin lookalikes.

            case 0x0041: return "a";
            case 0x0061: return "a";

            case 0x0042: return "b";
            case 0x0062: return "b";

            case 0x0043: return "c";
            case 0x0063: return "c";

            case 0x0044: return "d";
            case 0x0064: return "d";

            case 0x0045: return "e";
            case 0x0065: return "e";

            case 0x0047: return "g";
            case 0x0067: return "g";

            case 0x0048: return "h";
            case 0x0068: return "h";

            case 0x0049: return "i";
            case 0x0069: return "i";

            case 0x004A: return "j";
            case 0x006A: return "j";

            case 0x004B: return "k";
            case 0x006B: return "k";

            case 0x004D: return "m";
            case 0x006D: return "m";

            case 0x004E: return "n";
            case 0x006E: return "n";

            case 0x004F: return "o";
            case 0x006F: return "o";

            case 0x0050: return "p";
            case 0x0070: return "p";

            case 0x0052: return "r";
            case 0x0072: return "r";

            case 0x0053: return "s";
            case 0x0073: return "s";

            case 0x0054: return "t";
            case 0x0074: return "t";

            case 0x0055: return "u";
            case 0x0075: return "u";

            case 0x0056: return "v";
            case 0x0076: return "v";

            case 0x0058: return "x";
            case 0x0078: return "x";

            case 0x0059: return "y";
            case 0x0079: return "y";

            case 0x005A: return "z";
            case 0x007A: return "z";

            // Cyrillic lookalikes.

            case 0x0430: return "a"; // а
            case 0x0410: return "a"; // А

            case 0x0435: return "e"; // е
            case 0x0415: return "e"; // Е

            case 0x043E: return "o"; // о
            case 0x041E: return "o"; // О

            case 0x0440: return "p"; // р
            case 0x0420: return "p"; // Р

            case 0x0441: return "c"; // с
            case 0x0421: return "c"; // С

            case 0x0445: return "x"; // х
            case 0x0425: return "x"; // Х

            case 0x0443: return "y"; // у
            case 0x0423: return "y"; // У

            case 0x043A: return "k"; // к
            case 0x041A: return "k"; // К

            case 0x043C: return "m"; // м
            case 0x041C: return "m"; // М

            case 0x0442: return "t"; // т
            case 0x0422: return "t"; // Т

            case 0x0456: return "i"; // і
            case 0x0406: return "i"; // І

            case 0x0458: return "j"; // ј
            case 0x0408: return "j"; // Ј

            case 0x043D: return "h"; // н
            case 0x041D: return "h"; // Н

            case 0x0432: return "b"; // в
            case 0x0412: return "b"; // В

            // Greek lookalikes.

            case 0x0391: return "a"; // Α
            case 0x03B1: return "a"; // α

            case 0x0392: return "b"; // Β
            case 0x03B2: return "b"; // β

            case 0x0395: return "e"; // Ε
            case 0x03B5: return "e"; // ε

            case 0x0397: return "h"; // Η
            case 0x03B7: return "h"; // η

            case 0x0399: return "i"; // Ι
            case 0x03B9: return "i"; // ι

            case 0x039A: return "k"; // Κ
            case 0x03BA: return "k"; // κ

            case 0x039C: return "m"; // Μ
            case 0x03BC: return "m"; // μ

            case 0x039D: return "n"; // Ν
            case 0x03BD: return "n"; // ν

            case 0x039F: return "o"; // Ο
            case 0x03BF: return "o"; // ο

            case 0x03A1: return "p"; // Ρ
            case 0x03C1: return "p"; // ρ

            case 0x03A4: return "t"; // Τ
            case 0x03C4: return "t"; // τ

            case 0x03A7: return "x"; // Χ
            case 0x03C7: return "x"; // χ

            case 0x03A5: return "y"; // Υ
            case 0x03C5: return "y"; // υ

            // Digits and common ASCII confusables.

            case 0x0030: return "o";
            case 0x0031: return "l";
            case 0x0032: return "z";
            case 0x0035: return "s";
            case 0x0036: return "g";
            case 0x0038: return "b";

            // Common Unicode punctuation confusables.

            case 0x2010: return "-";
            case 0x2011: return "-";
            case 0x2012: return "-";
            case 0x2013: return "-";
            case 0x2014: return "-";

            case 0x2212: return "-";

            case 0xFF0D: return "-";

            case 0xFF21: return "a";
            case 0xFF22: return "b";
            case 0xFF23: return "c";
            case 0xFF24: return "d";
            case 0xFF25: return "e";
            case 0xFF26: return "f";
            case 0xFF27: return "g";
            case 0xFF28: return "h";
            case 0xFF29: return "i";
            case 0xFF2A: return "j";
            case 0xFF2B: return "k";
            case 0xFF2C: return "l";
            case 0xFF2D: return "m";
            case 0xFF2E: return "n";
            case 0xFF2F: return "o";
            case 0xFF30: return "p";
            case 0xFF31: return "q";
            case 0xFF32: return "r";
            case 0xFF33: return "s";
            case 0xFF34: return "t";
            case 0xFF35: return "u";
            case 0xFF36: return "v";
            case 0xFF37: return "w";
            case 0xFF38: return "x";
            case 0xFF39: return "y";
            case 0xFF3A: return "z";

            case 0xFF41: return "a";
            case 0xFF42: return "b";
            case 0xFF43: return "c";
            case 0xFF44: return "d";
            case 0xFF45: return "e";
            case 0xFF46: return "f";
            case 0xFF47: return "g";
            case 0xFF48: return "h";
            case 0xFF49: return "i";
            case 0xFF4A: return "j";
            case 0xFF4B: return "k";
            case 0xFF4C: return "l";
            case 0xFF4D: return "m";
            case 0xFF4E: return "n";
            case 0xFF4F: return "o";
            case 0xFF50: return "p";
            case 0xFF51: return "q";
            case 0xFF52: return "r";
            case 0xFF53: return "s";
            case 0xFF54: return "t";
            case 0xFF55: return "u";
            case 0xFF56: return "v";
            case 0xFF57: return "w";
            case 0xFF58: return "x";
            case 0xFF59: return "y";
            case 0xFF5A: return "z";

            default:
                break;
        }

        if (codePoint < 128) {
            char c =
                static_cast<char>(codePoint);

            if (std::isalpha(
                    static_cast<unsigned char>(c))) {
                return std::string(
                    1,
                    static_cast<char>(
                        std::tolower(
                            static_cast<unsigned char>(c)
                        )
                    )
                );
            }

            return std::string(1, c);
        }

        return "";
    }

    static std::string makeSkeleton(
        std::string_view identifier
    ) {
        std::string skeleton;

        std::size_t index = 0;

        while (index < identifier.size()) {
            std::uint32_t codePoint = 0;

            if (!decodeUTF8(
                    identifier,
                    index,
                    codePoint)) {

                return std::string();
            }

            std::string mapped =
                mapCodePoint(codePoint);

            if (!mapped.empty()) {
                skeleton += mapped;
            } else {
                appendUTF8(
                    skeleton,
                    codePoint
                );
            }
        }

        return skeleton;
    }
};

} // namespace SEC
} // namespace Shift