// MusicXML Class Library
// Copyright (c) by Matthew James Briggs
// Distributed under the MIT License

#include "mxtest/control/CompileControl.h"
#ifdef MX_COMPILE_CORE_TESTS

#include "DocumentPartwiseCreate.h"
#include "mx/core/Document.h"
#include "mx/core/Elements.h"
#include <sstream>

using namespace mx::core;

namespace mxtest
{
    mx::core::DocumentPtr createDocumentPartwise()
    {
        auto doc = makeDocument( DocumentChoice::partwise );
        
        /* Set Version Attribute on Score Element */
        auto s = doc->getScorePartwise();
        s->getAttributes()->hasVersion = true;
        s->getAttributes()->version = XsToken( "3.0" );
        
        /* Create Score Title and Credits */
        auto header = s->getScoreHeaderGroup();
        auto composerCredit = makeCredit();
        composerCredit->getCreditChoice()->setChoice( CreditChoice::Choice::creditWords );
        auto words = makeCreditWordsGroup();
        words->getCreditWords()->setValue( XsString( "Matthew James Briggs" ) );
        composerCredit->getCreditChoice()->addCreditWordsGroup( words );
        auto creditType = makeCreditType();
        creditType->setValue( XsString( "composer" ) );
        composerCredit->addCreditType( creditType );
        header->addCredit( composerCredit );
        header->setHasWork( true );
        header->getWork()->setHasWorkTitle( true );
        header->getWork()->getWorkTitle()->setValue( XsString( "Simple Measures" ) );
        
        /* Create Score Header First Part */
        header->getPartList()->getScorePart()->getAttributes()->id = XsID( "PARTONE" );
        header->getPartList()->getScorePart()->getPartName()->setValue( XsString( "Part One" ) );
        
        /* Create Score Header Second Part */
        auto p2 = makeScorePart();
        p2->getAttributes()->id = XsID( "A2" );
        p2->getPartName()->setValue( XsString( "Part Two" ) );
        auto p2g = makePartGroupOrScorePart();
        p2g->setChoice( PartGroupOrScorePart::Choice::scorePart );
        p2g->setScorePart( p2 );
        header->getPartList()->addPartGroupOrScorePart( p2g );
        
        /* Create Score Header Third Part */
        auto p3 = makeScorePart();
        p3->getAttributes()->id = XsID( "P3" );
        p3->getPartName()->setValue( XsString( "Part Three" ) );
        auto p3g = makePartGroupOrScorePart();
        p3g->setChoice( PartGroupOrScorePart::Choice::scorePart );
        p3g->setScorePart( p3 );
        header->getPartList()->addPartGroupOrScorePart( p3g );
        
        /* Create Three Partwise Part Stubs */
        // first part stub already exists
        s->addPartwisePart( makePartwisePart() ); // two
        s->addPartwisePart( makePartwisePart() ); // three
        
        auto iter = s->getPartwisePartSet().cbegin();
        auto part1 = *iter;
        auto part2 = *( ++iter );
        auto part3 = *( ++iter );

        part1->getAttributes()->id = XsIDREF( "PARTONE" );
        part2->getAttributes()->id = XsIDREF( "A2" );
        part3->getAttributes()->id = XsIDREF( "P3" );
        
        /* Add Three Measures to Part 1 */
        auto p1m1 = *( part1->getPartwiseMeasureSet().cbegin() );
        setPartwiseMeasureProperties( p1m1, // measure pointer
                                      1,    // measure number
                                      1,    // divisions
                                      4,    // beats
                                      4 );  // beat type
        addP1M1Data( p1m1->getMusicDataGroup() );
        
        auto p1m2 = makePartwiseMeasure();
        setPartwiseMeasureProperties( p1m2, // measure pointer
                                     2,    // measure number
                                     1,    // divisions
                                     4,    // beats
                                     4 );  // beat type
        addP1M2Data( p1m2->getMusicDataGroup() );
        part1->addPartwiseMeasure( p1m2 );
        
        auto p1m3 = makePartwiseMeasure();
        setPartwiseMeasureProperties( p1m3, // measure pointer
                                     3,    // measure number
                                     1,    // divisions
                                     2,    // beats
                                     4 );  // beat type
        addP1M3Data( p1m3->getMusicDataGroup() );
        part1->addPartwiseMeasure( p1m3 );
        
        /* Add Three Measures to Part 2 */
        auto p2m1 = *( part2->getPartwiseMeasureSet().cbegin() );
        setPartwiseMeasureProperties( p2m1, // measure pointer
                                     1,    // measure number
                                     1,    // divisions
                                     4,    // beats
                                     4 );  // beat type
        addP2M1Data( p2m1->getMusicDataGroup() );
        
        auto p2m2 = makePartwiseMeasure();
        setPartwiseMeasureProperties( p2m2, // measure pointer
                                     2,    // measure number
                                     1,    // divisions
                                     4,    // beats
                                     4 );  // beat type
        addP2M2Data( p2m2->getMusicDataGroup() );
        part2->addPartwiseMeasure( p2m2 );
        
        auto p2m3 = makePartwiseMeasure();
        setPartwiseMeasureProperties( p2m3, // measure pointer
                                     3,    // measure number
                                     1,    // divisions
                                     2,    // beats
                                     4 );  // beat type
        addP2M3Data( p2m3->getMusicDataGroup() );
        part2->addPartwiseMeasure( p2m3 );
        
        /* Add Three Measures to Part 3 */
        auto p3m1 = *( part3->getPartwiseMeasureSet().cbegin() );
        setPartwiseMeasureProperties( p3m1, // measure pointer
                                     1,    // measure number
                                     1,    // divisions
                                     4,    // beats
                                     4 );  // beat type
        addP3M1Data( p3m1->getMusicDataGroup() );
        
        auto p3m2 = makePartwiseMeasure();
        setPartwiseMeasureProperties( p3m2, // measure pointer
                                     2,    // measure number
                                     1,    // divisions
                                     4,    // beats
                                     4 );  // beat type
        addP3M2Data( p3m2->getMusicDataGroup() );
        part3->addPartwiseMeasure( p3m2 );
        
        auto p3m3 = makePartwiseMeasure();
        setPartwiseMeasureProperties( p3m3, // measure pointer
                                     3,    // measure number
                                     1,    // divisions
                                     2,    // beats
                                     4 );  // beat type
        addP3M3Data( p3m3->getMusicDataGroup() );
        part3->addPartwiseMeasure( p3m3 );
        
        return doc;
    }
    
    void setPartwiseMeasureProperties( PartwiseMeasurePtr& measure,
                                      int measureNumber,
                                      int divisions,
                                      int beats,
                                      int beatType )
    {
        std::stringstream measureNumberSstr;
        measureNumberSstr << measureNumber;
        std::stringstream beatSstr;
        beatSstr << beats;
        std::stringstream beatTypeSstr;
        beatTypeSstr << beatType;
        measure->getAttributes()->number = XsToken( measureNumberSstr.str() );
        auto propertiesChoice = makeMusicDataChoice();
        propertiesChoice->setChoice( MusicDataChoice::Choice::properties );
        auto properties = propertiesChoice->getProperties();
        properties->setHasDivisions( true );
        properties->getDivisions()->setValue( PositiveDivisionsValue( divisions ) );
        properties->addKey( makeKey() );
        auto time = makeTime();
        time->getTimeChoice()->setChoice( TimeChoice::Choice::timeSignature );
        auto timeSignature = makeTimeSignatureGroup();
        timeSignature->getBeats()->setValue( XsString( beatSstr.str() ) );
        timeSignature->getBeatType()->setValue( XsString( beatTypeSstr.str() ) );
        time->getTimeChoice()->addTimeSignatureGroup( timeSignature );
        time->getTimeChoice()->removeTimeSignatureGroup( time->getTimeChoice()->getTimeSignatureGroupSet().cbegin() );
        properties->addTime( time );
        auto clef = makeClef();
        clef->getSign()->setValue( ClefSign::g );
        clef->setHasLine( true );
        clef->getLine()->setValue( StaffLine( 2 ) );
        properties->addClef( clef );
        measure->getMusicDataGroup()->addMusicDataChoice( propertiesChoice );
        
    }
    
    MusicDataChoicePtr makeNote(
        StepEnum step,
        int octave,
        NoteTypeValue duration,
        int divisions )
    {
        auto p1m1_noteData = makeMusicDataChoice();
        p1m1_noteData->setChoice( MusicDataChoice::Choice::note );
        p1m1_noteData->getNote()->getNoteChoice()->setChoice( NoteChoice::Choice::normal );
        p1m1_noteData->getNote()->getNoteChoice()->getNormalNoteGroup()->getFullNoteGroup()->getFullNoteTypeChoice()->setChoice( FullNoteTypeChoice::Choice::pitch );
        p1m1_noteData->getNote()->getNoteChoice()->getNormalNoteGroup()->getFullNoteGroup()->getFullNoteTypeChoice()->getPitch()->getStep()->setValue( step );
        p1m1_noteData->getNote()->getNoteChoice()->getNormalNoteGroup()->getFullNoteGroup()->getFullNoteTypeChoice()->getPitch()->getOctave()->setValue( OctaveValue( octave ) );
        p1m1_noteData->getNote()->getNoteChoice()->getNormalNoteGroup()->getDuration()->setValue( PositiveDivisionsValue( divisions ) );
        p1m1_noteData->getNote()->getType()->setValue( duration );
        return p1m1_noteData;
    }
    
    void addP1M1Data( const MusicDataGroupPtr& musicDataGroup )
    {
        auto p1m1_noteData = makeNote( StepEnum::c,          // step
                                      4,                    // octave
                                      NoteTypeValue::whole, // duration
                                      4 );                  // divisions
        musicDataGroup->addMusicDataChoice( p1m1_noteData );
    }
    void addP1M2Data( const MusicDataGroupPtr& musicDataGroup )
    {
        auto p1m2_noteData = makeNote( StepEnum::d,          // step
                                      4,                    // octave
                                      NoteTypeValue::whole, // duration
                                      4 );                  // divisions
        musicDataGroup->addMusicDataChoice( p1m2_noteData );
    }
    void addP1M3Data( const MusicDataGroupPtr& musicDataGroup )
    {
        auto p1m3_noteData = makeNote( StepEnum::e,          // step
                                      4,                    // octave
                                      NoteTypeValue::half, // duration
                                      2 );                  // divisions
        musicDataGroup->addMusicDataChoice( p1m3_noteData );
    }
    void addP2M1Data( const MusicDataGroupPtr& musicDataGroup )
    {
        auto p2m1_noteData = makeNote( StepEnum::c,          // step
                                      5,                    // octave
                                      NoteTypeValue::quarter, // duration
                                      1 );                  // divisions
        musicDataGroup->addMusicDataChoice( p2m1_noteData );
        p2m1_noteData = makeNote( StepEnum::b,          // step
                                 4,                    // octave
                                 NoteTypeValue::quarter, // duration
                                 1 );                  // divisions
        musicDataGroup->addMusicDataChoice( p2m1_noteData );
        p2m1_noteData = makeNote( StepEnum::a,          // step
                                 4,                    // octave
                                 NoteTypeValue::quarter, // duration
                                 1 );                  // divisions
        musicDataGroup->addMusicDataChoice( p2m1_noteData );
        p2m1_noteData = makeNote( StepEnum::g,          // step
                                 4,                    // octave
                                 NoteTypeValue::quarter, // duration
                                 1 );                  // divisions
        musicDataGroup->addMusicDataChoice( p2m1_noteData );
    }
    void addP2M2Data( const MusicDataGroupPtr& musicDataGroup )
    {
        auto notedata = makeNote( StepEnum::a,          // step
                                      4,                    // octave
                                      NoteTypeValue::whole, // duration
                                      4 );                  // divisions
        musicDataGroup->addMusicDataChoice( notedata );
    }
    void addP2M3Data( const MusicDataGroupPtr& musicDataGroup )
    {
        auto notedata = makeNote( StepEnum::a,          // step
                                 4,                    // octave
                                 NoteTypeValue::quarter, // duration
                                 1 );                  // divisions
        musicDataGroup->addMusicDataChoice( notedata );
        notedata = makeNote( StepEnum::f,          // step
                                 4,                    // octave
                                 NoteTypeValue::quarter, // duration
                                 1 );                  // divisions
        musicDataGroup->addMusicDataChoice( notedata );
    }
    void addP3M1Data( const MusicDataGroupPtr& musicDataGroup )
    {
        auto notedata = makeNote( StepEnum::a,          // step
                                 3,                    // octave
                                 NoteTypeValue::quarter, // duration
                                 1 );                  // divisions
        musicDataGroup->addMusicDataChoice( notedata );
        notedata = makeNote( StepEnum::f,          // step
                            3,                    // octave
                            NoteTypeValue::quarter, // duration
                            1 );                  // divisions
        musicDataGroup->addMusicDataChoice( notedata );
        notedata = makeNote( StepEnum::g,          // step
                            3,                    // octave
                            NoteTypeValue::quarter, // duration
                            1 );                  // divisions
        musicDataGroup->addMusicDataChoice( notedata );
        notedata = makeNote( StepEnum::a,          // step
                            3,                    // octave
                            NoteTypeValue::quarter, // duration
                            1 );                  // divisions
        musicDataGroup->addMusicDataChoice( notedata );
    }
    void addP3M2Data( const MusicDataGroupPtr& musicDataGroup )
    {
        auto notedata = makeNote( StepEnum::b,          // step
                                 3,                    // octave
                                 NoteTypeValue::quarter, // duration
                                 1 );                  // divisions
        musicDataGroup->addMusicDataChoice( notedata );
        notedata = makeNote( StepEnum::c,          // step
                            4,                    // octave
                            NoteTypeValue::quarter, // duration
                            1 );                  // divisions
        musicDataGroup->addMusicDataChoice( notedata );
        notedata = makeNote( StepEnum::b,          // step
                            3,                    // octave
                            NoteTypeValue::quarter, // duration
                            1 );                  // divisions
        musicDataGroup->addMusicDataChoice( notedata );
        notedata = makeNote( StepEnum::a,          // step
                            3,                    // octave
                            NoteTypeValue::quarter, // duration
                            1 );                  // divisions
        musicDataGroup->addMusicDataChoice( notedata );
    }
    void addP3M3Data( const MusicDataGroupPtr& musicDataGroup )
    {
        auto notedata = makeNote( StepEnum::g,          // step
                                 3,                    // octave
                                 NoteTypeValue::quarter, // duration
                                 1 );                  // divisions
        musicDataGroup->addMusicDataChoice( notedata );
        notedata = makeNote( StepEnum::f,          // step
                            3,                    // octave
                            NoteTypeValue::quarter, // duration
                            1 );                  // divisions
        musicDataGroup->addMusicDataChoice( notedata );
    }
}

#endif
