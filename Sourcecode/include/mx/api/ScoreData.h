// MusicXML Class Library
// Copyright (c) by Matthew James Briggs
// Distributed under the MIT License

#pragma once

#include "mx/api/ApiCommon.h"
#include "mx/api/EncodingData.h"
#include "mx/api/DefaultsData.h"
#include "mx/api/PageData.h"
#include "mx/api/PageTextData.h"
#include "mx/api/PartData.h"
#include "mx/api/PartGroupData.h"
#include "mx/api/SystemData.h"
#include "mx/api/XXBadName.h"

#include <vector>
#include <set>

namespace mx
{
    namespace api
    {
        enum class MusicXmlVersion
        {
             unspecified,
             ThreePointZero,
        };

        class ScoreData
        {
        public:
            ScoreData();
            MusicXmlVersion musicXmlVersion;
            std::string musicXmlType;
            std::string workTitle;
            std::string workNumber;
            std::string movementTitle;
            std::string movementNumber;
            std::string composer;
            std::string lyricist;
            std::string arranger;
            std::string publisher;
            std::string copyright;
            EncodingData encoding;
            std::vector<PageTextData> pageTextItems;
            DefaultsData defaults;
            std::vector<PartData> parts;
            std::vector<PartGroupData> partGroups;
            int ticksPerQuarter;

            // TODO - document and rename
            std::map<MeasureIndex, XXBadName> xxbadnames;

            int getNumMeasures() const;
            int getNumStavesPerSystem() const;

            /// sorts all of the events, directions, etc.
            /// it is good to call this before writing to xml.
            void sort();
        };
        
        std::vector<PartData>::iterator findPart( std::vector<PartData>& inParts, const std::string& inPartId );
        
        MXAPI_EQUALS_BEGIN( ScoreData )
        MXAPI_EQUALS_MEMBER( musicXmlVersion )
        MXAPI_EQUALS_MEMBER( musicXmlType )
        MXAPI_EQUALS_MEMBER( workTitle )
        MXAPI_EQUALS_MEMBER( workNumber )
        MXAPI_EQUALS_MEMBER( movementTitle )
        MXAPI_EQUALS_MEMBER( movementNumber )
        MXAPI_EQUALS_MEMBER( composer )
        MXAPI_EQUALS_MEMBER( lyricist )
        MXAPI_EQUALS_MEMBER( arranger )
        MXAPI_EQUALS_MEMBER( publisher )
        MXAPI_EQUALS_MEMBER( copyright )
        MXAPI_EQUALS_MEMBER( encoding )
        MXAPI_EQUALS_MEMBER( pageTextItems )
        MXAPI_EQUALS_MEMBER( defaults )
        MXAPI_EQUALS_MEMBER( parts )
        MXAPI_EQUALS_MEMBER( partGroups )
        MXAPI_EQUALS_MEMBER( ticksPerQuarter )
        MXAPI_EQUALS_MEMBER( xxbadnames )
        MXAPI_EQUALS_END;
        MXAPI_NOT_EQUALS_AND_VECTORS( ScoreData );
    }
}
