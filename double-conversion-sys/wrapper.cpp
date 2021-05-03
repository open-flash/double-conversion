#include <double-conversion/double-conversion.h>
#include "wrapper.hpp"

namespace double_conversion {
    bool DoubleToStringConverter_ToShortest(const DoubleToStringConverter& self, double value, StringBuilder* result_builder) {
        return self.ToShortest(value, result_builder);
    }

    char* StringBuilder_Finalize(StringBuilder& self) {
        return self.Finalize();
    }
}
