module Paths_real (
    version,
    getBinDir, getLibDir, getDataDir, getLibexecDir,
    getDataFileName, getSysconfDir
  ) where

import qualified Control.Exception as Exception
import Data.Version (Version(..))
import System.Environment (getEnv)
import Prelude

catchIO :: IO a -> (Exception.IOException -> IO a) -> IO a
catchIO = Exception.catch

version :: Version
version = Version [0,1,0,0] []
bindir, libdir, datadir, libexecdir, sysconfdir :: FilePath

bindir     = "/home/pilot114/sources/haskell/real/.stack-work/install/x86_64-linux/lts-6.3/7.10.3/bin"
libdir     = "/home/pilot114/sources/haskell/real/.stack-work/install/x86_64-linux/lts-6.3/7.10.3/lib/x86_64-linux-ghc-7.10.3/real-0.1.0.0-GrpX4K99dRYEEwkWysvzfy"
datadir    = "/home/pilot114/sources/haskell/real/.stack-work/install/x86_64-linux/lts-6.3/7.10.3/share/x86_64-linux-ghc-7.10.3/real-0.1.0.0"
libexecdir = "/home/pilot114/sources/haskell/real/.stack-work/install/x86_64-linux/lts-6.3/7.10.3/libexec"
sysconfdir = "/home/pilot114/sources/haskell/real/.stack-work/install/x86_64-linux/lts-6.3/7.10.3/etc"

getBinDir, getLibDir, getDataDir, getLibexecDir, getSysconfDir :: IO FilePath
getBinDir = catchIO (getEnv "real_bindir") (\_ -> return bindir)
getLibDir = catchIO (getEnv "real_libdir") (\_ -> return libdir)
getDataDir = catchIO (getEnv "real_datadir") (\_ -> return datadir)
getLibexecDir = catchIO (getEnv "real_libexecdir") (\_ -> return libexecdir)
getSysconfDir = catchIO (getEnv "real_sysconfdir") (\_ -> return sysconfdir)

getDataFileName :: FilePath -> IO FilePath
getDataFileName name = do
  dir <- getDataDir
  return (dir ++ "/" ++ name)
