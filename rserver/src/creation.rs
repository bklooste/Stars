// namespace FreeStars {
//
// /**
//  * Game creation settings.
//  * This class contains all the settings passed to the galaxy generator.
//  */
// class Creation {
// public:
// 	Creation();
// 	~Creation();
//
// 	bool LoadCreation(const TiXmlNode * options);
// 	bool LoadNames(const char * NameFile);
// 	/**
// 	 * Planet placement function.
// 	 * Overload this function if you need to change the planet placement algorithm.
// 	 */
// 	void SetLocation(Planet * p);
//
// 	long mWorlds;	///< Number of planets to generate
//
// 	long mMinDistance;			///< Minimum distance between planets.
// 	/**
// 	 * Cluster odds.
// 	 * Odds of planet cluster forming.
// 	 * Must be in range 0--1.
// 	 */
// 	double mClusterOdds;
// 	/**
// 	 * Cluster odds falloff.
// 	 * Rate of degrading in cluster odds after successfully starting a cluster.
// 	 * A higher value means clusters with less planets.
// 	 * Each planet created in the cluster decreases the cluster odds by this much.
// 	 * Should be greater than 0 and less than mClusterOdds.
// 	 */
// 	double mClusterOddsDegrade; ///< Rate of degrading in the odds.
// 	/**
// 	 * Current cluster odds.
// 	 * Set to 1.0 to force an initial cluster around the initial planet.
// 	 */
// 	double mCurrClusterOdds;
// 	long mClusterMaxDistance;	///< Maximum size of a cluster
//
// 	long mStartPositionCount;
// 	long mStartPositionMin;
// 	long mStartPositionMax;
//
// 	long mHWBasePop;
// 	long mHWPopBonus;
// 	long mHWFactories;
// 	long mHWMines;
// 	long mHWDefenses;
//
// 	bool mSecondaryWorlds;
// 	double mPopMultiplierFor2nd;
// 	double mSecondaryPop;
// 	long mSWFactories;
// 	long mSWMines;
// 	long mSWDefenses;growth: i32,
    
// 	long mSWMaxMin;
// 	long mSWMinMin;
// 	long mMinSWDistance;growth: i32,

// 	long mMaxSWDistance;
//
// 	string GetNextName();
// 	void AddHW(Planet * p)	{ mPrePlacedHWs.push_back(p); }
// 	Planet * GetNextHW()	{ return mHW == mPrePlacedHWs.end() ? NULL : *mHW++; }
// 	Planet * GetSecond(const Player * p);
// 	void AddSecond(const Player * p, Planet * sw)	{ mSecondWorlds.insert(pair<const Player *, Planet *>(p, sw)); }
//
// private:
// 	deque<string> mNames;
// 	deque<string>::iterator mNamePos;
// 	long mNameCount;
// 	deque<Planet *> mPrePlacedHWs;
// 	deque<Planet *>::iterator mHW;
// 	map<const Player *, Planet *> mSecondWorlds;
//
// 	Planet * LastP;
// };
// }
// #endif // !defined(FreeStars_Creation_h)
