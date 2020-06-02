// the main include that always needs to be included in every translation unit that uses the PSD library
#include "vendor/src/Psd/Psd.h"

// for convenience reasons, we directly include the platform header from the PSD library.
// we could have just included <Windows.h> as well, but that is unnecessarily big, and triggers lots of warnings.
#include "vendor/src/Psd/PsdPlatform.h"

// in the sample, we use the provided malloc allocator for all memory allocations. likewise, we also use the provided
// native file interface.
// in your code, feel free to use whatever allocator you have lying around.
#include "vendor/src/Psd/PsdMallocAllocator.h"
#include "vendor/src/Psd/PsdNativeFile.h"

#include "vendor/src/Psd/PsdDocument.h"
#include "vendor/src/Psd/PsdColorMode.h"
#include "vendor/src/Psd/PsdLayer.h"
#include "vendor/src/Psd/PsdChannel.h"
#include "vendor/src/Psd/PsdChannelType.h"
#include "vendor/src/Psd/PsdLayerMask.h"
#include "vendor/src/Psd/PsdVectorMask.h"
#include "vendor/src/Psd/PsdLayerMaskSection.h"
#include "vendor/src/Psd/PsdImageDataSection.h"
#include "vendor/src/Psd/PsdImageResourcesSection.h"
#include "vendor/src/Psd/PsdParseDocument.h"
#include "vendor/src/Psd/PsdParseLayerMaskSection.h"
#include "vendor/src/Psd/PsdParseImageDataSection.h"
#include "vendor/src/Psd/PsdParseImageResourcesSection.h"
#include "vendor/src/Psd/PsdLayerCanvasCopy.h"
#include "vendor/src/Psd/PsdInterleave.h"
#include "vendor/src/Psd/PsdPlanarImage.h"
#include "vendor/src/Psd/PsdExport.h"
#include "vendor/src/Psd/PsdExportDocument.h"
